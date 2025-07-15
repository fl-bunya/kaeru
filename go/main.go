package main

import (
	"fmt"
	"log"
	"net/http"
	"net/url"
	"strconv"
	"strings"
	"time"

	"github.com/PuerkitoBio/goquery"
)

type RouteInfo struct {
	DepartureTime string
	ArrivalTime   string
	Duration      string
	Transfers     string
	Fare          string
	Route         []string
}

type TimeParams struct {
	Y  string
	M  string
	D  string
	HH string
	M1 string
	M2 string
}

func getCurrentTimeParams() TimeParams {
	now := time.Now()
	year := strconv.Itoa(now.Year())
	month := fmt.Sprintf("%02d", now.Month())
	day := fmt.Sprintf("%02d", now.Day())
	hour := fmt.Sprintf("%02d", now.Hour())
	minute := fmt.Sprintf("%02d", now.Minute())

	return TimeParams{
		Y:  year,
		M:  month,
		D:  day,
		HH: hour,
		M1: string(minute[0]),
		M2: string(minute[1]),
	}
}

func buildYahooTransitURL(timeParams TimeParams) string {
	baseURL := "https://transit.yahoo.co.jp/search/result"
	params := url.Values{}
	
	params.Set("from", "新富町(東京都)")
	params.Set("to", "上福岡")
	params.Set("fromgid", "")
	params.Set("togid", "")
	params.Set("flatlon", "")
	params.Set("tlatlon", "")
	params.Set("y", timeParams.Y)
	params.Set("m", timeParams.M)
	params.Set("d", timeParams.D)
	params.Set("hh", timeParams.HH)
	params.Set("m1", timeParams.M1)
	params.Set("m2", timeParams.M2)
	params.Set("type", "1")
	params.Set("ticket", "ic")
	params.Set("expkind", "1")
	params.Set("userpass", "1")
	params.Set("ws", "3")
	params.Set("s", "2")
	params.Set("al", "0")
	params.Set("shin", "0")
	params.Set("ex", "0")
	params.Set("hb", "1")
	params.Set("lb", "1")
	params.Set("sr", "1")

	return fmt.Sprintf("%s?%s", baseURL, params.Encode())
}

func fetchTransitRoutes() ([]RouteInfo, error) {
	timeParams := getCurrentTimeParams()
	url := buildYahooTransitURL(timeParams)
	
	fmt.Printf("Fetching transit routes from: %s\n", url)
	
	resp, err := http.Get(url)
	if err != nil {
		return nil, fmt.Errorf("HTTP request failed: %w", err)
	}
	defer resp.Body.Close()
	
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("HTTP error! status: %d", resp.StatusCode)
	}
	
	doc, err := goquery.NewDocumentFromReader(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to parse HTML: %w", err)
	}
	
	var routes []RouteInfo
	
	// 最初の3つのルートを取得
	doc.Find(".routeDetail").Slice(0, 3).Each(func(index int, s *goquery.Selection) {
		route := RouteInfo{}
		
		// 発車時刻と到着時刻
		route.DepartureTime = strings.TrimSpace(s.Find(".time_departure, .departureTime, .time").First().Text())
		route.ArrivalTime = strings.TrimSpace(s.Find(".time_arrival, .arrivalTime, .time").Last().Text())
		
		// 所要時間
		route.Duration = strings.TrimSpace(s.Find(".time_required, .duration, .timeInfo").Text())
		
		// 乗換回数
		route.Transfers = strings.TrimSpace(s.Find(".transfer_count, .transfers, .transfer").Text())
		
		// 運賃
		route.Fare = strings.TrimSpace(s.Find(".fare, .price, .cost").Text())
		
		// ルート詳細
		s.Find(".route_detail, .routeDetail, .transport, .line").Each(func(_ int, detail *goquery.Selection) {
			text := strings.TrimSpace(detail.Text())
			if text != "" {
				route.Route = append(route.Route, text)
			}
		})
		
		routes = append(routes, route)
	})
	
	return routes, nil
}

func main() {
	fmt.Println("🚇 Yahoo乗換案内からルート情報を取得中...")
	
	routes, err := fetchTransitRoutes()
	if err != nil {
		log.Printf("❌ ルート情報を取得できませんでした: %v", err)
		return
	}
	
	if len(routes) == 0 {
		fmt.Println("❌ ルート情報を取得できませんでした")
		return
	}
	
	fmt.Printf("\n📋 取得したルート情報 (%d件):\n\n", len(routes))
	
	for i, route := range routes {
		fmt.Printf("=== ルート %d ===\n", i+1)
		fmt.Printf("発車時刻: %s\n", route.DepartureTime)
		fmt.Printf("到着時刻: %s\n", route.ArrivalTime)
		// fmt.Printf("所要時間: %s\n", route.Duration)
		// fmt.Printf("乗換回数: %s\n", route.Transfers)
		// fmt.Printf("運賃: %s\n", route.Fare)
		
		if len(route.Route) > 0 {
			fmt.Printf("ルート: %s\n", strings.Join(route.Route, " → "))
		} else {
			fmt.Println("ルート: N/A")
		}
		fmt.Println()
	}
} 