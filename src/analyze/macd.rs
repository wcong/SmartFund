pub fn calculate_ema(prices: &[f64], period: usize) -> Vec<f64> {
    let mut ema = Vec::new();
    if prices.len() < period || period == 0 {
        return ema;
    }

    // Calculate the initial SMA and use it as the first EMA value
    let sma: f64 = prices[..period].iter().sum::<f64>() / period as f64;
    ema.push(sma);

    let multiplier = 2.0 / (period as f64 + 1.0);

    for price in &prices[period..] {
        let prev_ema = *ema.last().unwrap();
        let new_ema = (price - prev_ema) * multiplier + prev_ema;
        ema.push(new_ema);
    }

    ema
}

pub fn calculate_macd(prices: &[f64], short_period: usize, long_period: usize, signal_period: usize) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if prices.len() < long_period + signal_period {
        return None;
    }

    let ema_short = calculate_ema(prices, short_period);
    let ema_long = calculate_ema(prices, long_period);

    // Align the EMAs to the same starting point
    let offset = ema_long.len().min(ema_short.len());
    let macd_line: Vec<f64> = ema_short[ema_short.len() - offset..]
        .iter()
        .zip(&ema_long[ema_long.len() - offset..])
        .map(|(short, long)| short - long)
        .collect();

    let signal_line = calculate_ema(&macd_line, signal_period);

    // Histogram: MACD line - Signal line (align lengths)
    let hist_offset = macd_line.len().min(signal_line.len());
    let histogram: Vec<f64> = macd_line[macd_line.len() - hist_offset..]
        .iter()
        .zip(&signal_line[signal_line.len() - hist_offset..])
        .map(|(macd, signal)| macd - signal)
        .collect();

    Some((macd_line, signal_line, histogram))
}