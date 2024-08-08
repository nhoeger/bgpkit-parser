use bgpkit_parser::BgpkitParser;

fn main() {
    let mut otc_count = 0;
    let mut no_otc_count = 0;
    let mut iteration = 0;
    let base_url = "http://archive.routeviews.org/bgpdata/2024.08/RIBS/";
    let a = ["rib.20240801.0000.bz2", "rib.20240801.0200.bz2", "rib.20240801.0400.bz2", "rib.20240801.0600.bz2", "rib.20240801.0800.bz2", "rib.20240801.1000.bz2", "rib.20240801.1200.bz2", "rib.20240801.1400.bz2", "rib.20240801.1600.bz2", "rib.20240801.1800.bz2", "rib.20240801.2000.bz2", "rib.20240801.2200.bz2", "rib.20240802.0000.bz2", "rib.20240802.0200.bz2", "rib.20240802.0400.bz2", "rib.20240802.0600.bz2", "rib.20240802.0800.bz2", "rib.20240802.1000.bz2", "rib.20240802.1200.bz2", "rib.20240802.1400.bz2", "rib.20240802.1600.bz2", "rib.20240802.1800.bz2", "rib.20240802.2000.bz2", "rib.20240802.2200.bz2", "rib.20240803.0000.bz2", "rib.20240803.0200.bz2", "rib.20240803.0400.bz2", "rib.20240803.0600.bz2", "rib.20240803.0800.bz2", "rib.20240803.1000.bz2", "rib.20240803.1200.bz2", "rib.20240803.1400.bz2", "rib.20240803.1600.bz2", "rib.20240803.1800.bz2", "rib.20240803.2000.bz2", "rib.20240803.2200.bz2", "rib.20240804.0000.bz2", "rib.20240804.0200.bz2", "rib.20240804.0400.bz2", "rib.20240804.0600.bz2", "rib.20240804.0800.bz2", "rib.20240804.1000.bz2", "rib.20240804.1200.bz2", "rib.20240804.1400.bz2", "rib.20240804.1600.bz2", "rib.20240804.1800.bz2", "rib.20240804.2000.bz2", "rib.20240804.2200.bz2", "rib.20240805.0000.bz2", "rib.20240805.0200.bz2", "rib.20240805.0400.bz2", "rib.20240805.0600.bz2", "rib.20240805.0800.bz2", "rib.20240805.1000.bz2", "rib.20240805.1200.bz2", "rib.20240805.1400.bz2", "rib.20240805.1600.bz2", "rib.20240805.1800.bz2", "rib.20240805.2000.bz2", "rib.20240805.2200.bz2", "rib.20240806.0000.bz2", "rib.20240806.0200.bz2", "rib.20240806.0400.bz2", "rib.20240806.0600.bz2", "rib.20240806.0800.bz2", "rib.20240806.1000.bz2", "rib.20240806.1200.bz2", "rib.20240806.1400.bz2", "rib.20240806.1600.bz2", "rib.20240806.1800.bz2", "rib.20240806.2000.bz2", "rib.20240806.2200.bz2", "rib.20240807.0000.bz2", "rib.20240807.0200.bz2", "rib.20240807.0400.bz2", "rib.20240807.0600.bz2", "rib.20240807.0800.bz2", "rib.20240807.1000.bz2", "rib.20240807.1200.bz2", "rib.20240807.1400.bz2", "rib.20240807.1600.bz2", "rib.20240807.1800.bz2", "rib.20240807.2000.bz2", "rib.20240807.2200.bz2", "rib.20240808.0000.bz2", "rib.20240808.0200.bz2", "rib.20240808.0400.bz2", "rib.20240808.0600.bz2", "rib.20240808.0800.bz2", "rib.20240808.1000.bz2", "rib.20240808.1200.bz2", "rib.20240808.1400.bz2"];

    for filename in a {
        let url = format!("{}{}", base_url, filename);
        for elem in BgpkitParser::new(&url).unwrap() {
            if let Some(otc) = elem.only_to_customer {
                // println!("OTC found: {} for path {}\n{}\n", &otc, &elem.as_path.as_ref().unwrap(), &elem);
                otc_count += 1;
            } else {
                no_otc_count += 1;
            }
        }
        iteration += 1;
        let total_count = otc_count + no_otc_count;
        let otc_percentage = (otc_count as f64 / total_count as f64) * 100.0;
        println!("Done with {}/92 iterations. Current result {:.2}%", iteration, otc_percentage)
    }

    println!("Elements with OTC: {}", otc_count);
    println!("Elements without OTC: {}", no_otc_count);
}