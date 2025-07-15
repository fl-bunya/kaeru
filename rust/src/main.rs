use anyhow::{anyhow, Result};
use chrono::{Datelike, Timelike, Utc};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Debug)]
struct RouteInfo {
    departure_time: String,
    arrival_time: String,
    duration: String,
    transfers: String,
    fare: String,
    route: Vec<String>,
}

#[derive(Debug)]
struct TimeParams {
    y: String,
    m: String,
    d: String,
    hh: String,
    m1: String,
    m2: String,
}

fn get_current_time_params() -> TimeParams {
    let now = Utc::now();
    let year = now.year().to_string();
    let month = format!("{:02}", now.month());
    let day = format!("{:02}", now.day());
    let hour = format!("{:02}", now.hour());
    let minute = format!("{:02}", now.minute());

    TimeParams {
        y: year,
        m: month,
        d: day,
        hh: hour,
        m1: minute.chars().nth(0).unwrap().to_string(),
        m2: minute.chars().nth(1).unwrap().to_string(),
    }
}

fn build_yahoo_transit_url(time_params: &TimeParams) -> String {
    let base_url = "https://transit.yahoo.co.jp/search/result";
    let mut params = HashMap::new();
    
    params.insert("from", "新富町(東京都)");
    params.insert("to", "上福岡");
    params.insert("fromgid", "");
    params.insert("togid", "");
    params.insert("flatlon", "");
    params.insert("tlatlon", "");
    params.insert("y", &time_params.y);
    params.insert("m", &time_params.m);
    params.insert("d", &time_params.d);
    params.insert("hh", &time_params.hh);
    params.insert("m1", &time_params.m1);
    params.insert("m2", &time_params.m2);
    params.insert("type", "1");
    params.insert("ticket", "ic");
    params.insert("expkind", "1");
    params.insert("userpass", "1");
    params.insert("ws", "3");
    params.insert("s", "2");
    params.insert("al", "0");
    params.insert("shin", "0");
    params.insert("ex", "0");
    params.insert("hb", "1");
    params.insert("lb", "1");
    params.insert("sr", "1");

    let query_string: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    format!("{}?{}", base_url, query_string)
}

fn fetch_transit_routes() -> Result<Vec<RouteInfo>> {
    let time_params = get_current_time_params();
    let url = build_yahoo_transit_url(&time_params);
    
    println!("Fetching transit routes from: {}", url);
    
    let client = Client::new();
    let response = client.get(&url).send()?;
    
    if !response.status().is_success() {
        return Err(anyhow!("HTTP error! status: {}", response.status()));
    }
    
    let html = response.text()?;
    let document = Html::parse_document(&html);
    
    let mut routes = Vec::new();
    
    // 最初の3つのルートを取得
    let route_selector = Selector::parse(".routeDetail").unwrap();
    let route_elements: Vec<_> = document.select(&route_selector).take(3).collect();
    
    for element in route_elements {
        let mut route = RouteInfo {
            departure_time: String::new(),
            arrival_time: String::new(),
            duration: String::new(),
            transfers: String::new(),
            fare: String::new(),
            route: Vec::new(),
        };
        
        // 発車時刻と到着時刻
        let time_selectors = [
            ".time_departure",
            ".departureTime", 
            ".time"
        ];
        
        for selector_str in &time_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(first_time) = element.select(&selector).next() {
                    route.departure_time = first_time.text().collect::<String>().trim().to_string();
                    break;
                }
            }
        }
        
        for selector_str in &time_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(last_time) = element.select(&selector).last() {
                    route.arrival_time = last_time.text().collect::<String>().trim().to_string();
                    break;
                }
            }
        }
        
        // 所要時間
        let duration_selectors = [".time_required", ".duration", ".timeInfo"];
        for selector_str in &duration_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(duration) = element.select(&selector).next() {
                    route.duration = duration.text().collect::<String>().trim().to_string();
                    break;
                }
            }
        }
        
        // 乗換回数
        let transfer_selectors = [".transfer_count", ".transfers", ".transfer"];
        for selector_str in &transfer_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(transfers) = element.select(&selector).next() {
                    route.transfers = transfers.text().collect::<String>().trim().to_string();
                    break;
                }
            }
        }
        
        // 運賃
        let fare_selectors = [".fare", ".price", ".cost"];
        for selector_str in &fare_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(fare) = element.select(&selector).next() {
                    route.fare = fare.text().collect::<String>().trim().to_string();
                    break;
                }
            }
        }
        
        // ルート詳細
        let route_detail_selectors = [".route_detail", ".routeDetail", ".transport", ".line"];
        for selector_str in &route_detail_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for detail in element.select(&selector) {
                    let text = detail.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        route.route.push(text);
                    }
                }
                if !route.route.is_empty() {
                    break;
                }
            }
        }
        
        routes.push(route);
    }
    
    Ok(routes)
}

fn main() -> Result<()> {
    println!("🚇 Yahoo乗換案内からルート情報を取得中...");
    
    match fetch_transit_routes() {
        Ok(routes) => {
            if routes.is_empty() {
                println!("❌ ルート情報を取得できませんでした");
                return Ok(());
            }
            
            println!("\n📋 取得したルート情報 ({}件):\n", routes.len());
            
            for (i, route) in routes.iter().enumerate() {
                println!("=== ルート {} ===", i + 1);
                println!("発車時刻: {}", route.departure_time);
                println!("到着時刻: {}", route.arrival_time);
                // println!("所要時間: {}", route.duration);
                // println!("乗換回数: {}", route.transfers);
                // println!("運賃: {}", route.fare);
                
                if !route.route.is_empty() {
                    println!("ルート: {}", route.route.join(" → "));
                } else {
                    println!("ルート: N/A");
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("❌ ルート情報を取得できませんでした: {}", e);
        }
    }
    
    Ok(())
} 