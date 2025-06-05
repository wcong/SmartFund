pub fn calculate_rsi(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period + 1 || period == 0 {
        return None;
    }

    let mut gains = 0.0;
    let mut losses = 0.0;

    // Calculate initial average gain and loss
    for i in 1..=period {
        let diff = prices[i] - prices[i - 1];
        if diff > 0.0 {
            gains += diff;
        } else {
            losses -= diff;
        }
    }

    let mut avg_gain = gains / period as f64;
    let mut avg_loss = losses / period as f64;

    // Calculate RSI for the last value
    for i in (period + 1)..prices.len() {
        let diff = prices[i] - prices[i - 1];
        if diff > 0.0 {
            avg_gain = (avg_gain * (period as f64 - 1.0) + diff) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0)) / period as f64;
        } else {
            avg_gain = (avg_gain * (period as f64 - 1.0)) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0) - diff) / period as f64;
        }
    }

    if avg_loss == 0.0 {
        return Some(100.0);
    }

    let rs = avg_gain / avg_loss;
    let rsi = 100.0 - (100.0 / (1.0 + rs));
    Some(rsi)
}