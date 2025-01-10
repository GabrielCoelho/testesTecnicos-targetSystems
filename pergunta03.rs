use serde_json::Value;

fn main() {
    // Exemplo de JSON com os dados de invoicing
    let json_data = r#"
    {
        "invoicing": [1200.0, 0.0, 800.5, 0.0, 950.3, 1100.0, 0.0, 0.0, 1300.7, 1400.8, 0.0, 0.0, 1250.9, 0.0, 0.0, 1600.1, 1550.2, 1200.0, 0.0, 1000.5, 0.0, 1450.3, 1350.0, 0.0, 0.0, 1700.6, 0.0, 1500.4, 1400.0, 1600.0]
    }
    "#;

    // Parse do JSON
    let parsed: Value = serde_json::from_str(json_data).expect("Error parsing JSON");
    let invoicing = parsed["invoicing"]
        .as_array()
        .expect("JSON invalid format")
        .iter()
        .filter_map(|x| x.as_f64())
        .collect::<Vec<f64>>();

    // Filtrar dias válidos (com invoicing > 0)
    let valid_days: Vec<f64> = invoicing.iter().copied().filter(|&x| x > 0.0).collect();

    // Cálculos
    let least_invoice = valid_days.iter().cloned().fold(f64::INFINITY, f64::min);
    let greater_invoice = valid_days.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let total_sum: f64 = valid_days.iter().sum();
    let avg_monthly = total_sum / valid_days.len() as f64;

    let days_above_avg = valid_days.iter().filter(|&&x| x > avg_monthly).count();

    // Exibir resultados
    println!("Menor valor de invoicing: {:.2}", least_invoice);
    println!("Maior valor de invoicing: {:.2}", greater_invoice);
    println!("Número de dias acima da média mensal: {}", days_above_avg);
}
