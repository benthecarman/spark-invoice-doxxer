use lightning_invoice::Bolt11Invoice;
use spark::Network;
use spark::address::SparkAddress;

fn main() {
    // parse first argument as bolt11 invoice
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <bolt11_invoice>", args[0]);
        std::process::exit(1);
    }

    let invoice_str = &args[1];
    let decoded_invoice = match invoice_str.parse::<Bolt11Invoice>() {
        Ok(invoice) => invoice,
        Err(e) => {
            eprintln!("Failed to parse invoice: {}", e);
            std::process::exit(1);
        }
    };

    match extract_spark_address(&decoded_invoice) {
        Some(spark_address) => {
            println!("Extracted Spark Address: {spark_address}");
            if spark_address.network == Network::Mainnet {
                println!("https://www.sparkscan.io/address/{spark_address}?network=mainnet")
            }
        }
        None => eprintln!("No Spark Address found in the invoice"),
    }
}

const RECEIVER_IDENTITY_PUBLIC_KEY_SHORT_CHANNEL_ID: u64 = 17592187092992000001;
fn extract_spark_address(decoded_invoice: &Bolt11Invoice) -> Option<SparkAddress> {
    for route_hint in decoded_invoice.route_hints() {
        for node in route_hint.0 {
            if node.short_channel_id == RECEIVER_IDENTITY_PUBLIC_KEY_SHORT_CHANNEL_ID {
                return Some(SparkAddress::new(
                    node.src_node_id,
                    decoded_invoice.network().try_into().ok()?,
                    None,
                    None,
                ));
            }
        }
    }
    None
}
