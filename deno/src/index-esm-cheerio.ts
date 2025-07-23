import cheerio from "https://esm.sh/cheerio@1.0.0-rc.10";

interface RouteInfo {
  departureTime: string;
  arrivalTime: string;
  duration: string;
  transfers: string;
  fare: string;
  route: string[];
}

async function getCurrentTimeParams(): Promise<{ y: string; m: string; d: string; hh: string; m1: string; m2: string }> {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  const hour = String(now.getHours()).padStart(2, '0');
  const minute = String(now.getMinutes()).padStart(2, '0');
  
  return {
    y: String(year),
    m: month,
    d: day,
    hh: hour,
    m1: minute[0],
    m2: minute[1]
  };
}

function buildYahooTransitURL(timeParams: any): string {
  const baseURL = 'https://transit.yahoo.co.jp/search/result';
  const params = new URLSearchParams({
    from: '新富町(東京都)',
    to: '上福岡',
    fromgid: '',
    togid: '',
    flatlon: '',
    tlatlon: '',
    ...timeParams,
    type: '1',
    ticket: 'ic',
    expkind: '1',
    userpass: '1',
    ws: '3',
    s: '2',
    al: '0',
    shin: '0',
    ex: '0',
    hb: '1',
    lb: '1',
    sr: '1'
  });
  
  return `${baseURL}?${params.toString()}`;
}

async function fetchTransitRoutes(): Promise<RouteInfo[]> {
  try {
    const timeParams = await getCurrentTimeParams();
    const url = buildYahooTransitURL(timeParams);
    
    console.log(`Fetching transit routes from: ${url}`);
    
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    
    const html = await response.text();
    const $ = cheerio.load(html);
    
    const routes: RouteInfo[] = [];
    
    // 実際のYahoo乗換案内ページの構造に合わせてセレクタを調整
    $('.routeDetail').slice(0, 3).each((index: number, element: any) => {
      const $route = $(element);
      
      // 発車時刻と到着時刻（実際のHTML構造に基づいて調整）
      const departureTime = $route.find('.time_departure, .departureTime, .time').first().text().trim();
      const arrivalTime = $route.find('.time_arrival, .arrivalTime, .time').last().text().trim();
      
      // 所要時間
      const duration = $route.find('.time_required, .duration, .timeInfo').text().trim();
      
      // 乗換回数
      const transfers = $route.find('.transfer_count, .transfers, .transfer').text().trim();
      
      // 運賃
      const fare = $route.find('.fare, .price, .cost').text().trim();
      
      // ルート詳細
      const routeDetails: string[] = [];
      $route.find('.route_detail, .routeDetail, .transport, .line').each((_: number, detail: any) => {
        const text = $(detail).text().trim();
        if (text) {
          routeDetails.push(text);
        }
      });
      
      routes.push({
        departureTime,
        arrivalTime,
        duration,
        transfers,
        fare,
        route: routeDetails
      });
    });
    
    return routes;
  } catch (error) {
    console.error('Error fetching transit routes:', error);
    return [];
  }
}

async function main() {
  console.log('🚇 Yahoo乗換案内からルート情報を取得中...');
  
  const routes = await fetchTransitRoutes();
  
  if (routes.length === 0) {
    console.log('❌ ルート情報を取得できませんでした');
    return;
  }
  
  console.log(`\n📋 取得したルート情報 (${routes.length}件):\n`);
  
  routes.forEach((route, index) => {
    console.log(`=== ルート ${index + 1} ===`);
    console.log(`発車時刻: ${route.departureTime || 'N/A'}`);
    console.log(`到着時刻: ${route.arrivalTime || 'N/A'}`);
    console.log(`ルート: ${route.route.length > 0 ? route.route.join(' → ') : 'N/A'}`);
    console.log('');
  });
}

main().catch(console.error); 