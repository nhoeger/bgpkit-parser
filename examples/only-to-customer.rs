use bgpkit_parser::BgpkitParser;

fn main() {
    let mut total_otc_count: u64 = 0;
    let mut total_no_otc_count: u64 = 0;
    let mut otc_count: u64 = 0;
    let mut no_otc_count: u64 = 0;
    let mut iteration = 0;
    let base_url_rib_august = "https://archive.routeviews.org/bgpdata/2024.08/RIBS/";
    let base_url_rib_july = "https://archive.routeviews.org/bgpdata/2024.07/RIBS/";
    let a = ["rib.20240801.0000.bz2", "rib.20240801.0200.bz2", "rib.20240801.0400.bz2", "rib.20240801.0600.bz2", "rib.20240801.0800.bz2", "rib.20240801.1000.bz2", "rib.20240801.1200.bz2", "rib.20240801.1400.bz2", "rib.20240801.1600.bz2", "rib.20240801.1800.bz2", "rib.20240801.2000.bz2", "rib.20240801.2200.bz2", "rib.20240802.0000.bz2", "rib.20240802.0200.bz2", "rib.20240802.0400.bz2", "rib.20240802.0600.bz2", "rib.20240802.0800.bz2", "rib.20240802.1000.bz2", "rib.20240802.1200.bz2", "rib.20240802.1400.bz2", "rib.20240802.1600.bz2", "rib.20240802.1800.bz2", "rib.20240802.2000.bz2", "rib.20240802.2200.bz2", "rib.20240803.0000.bz2", "rib.20240803.0200.bz2", "rib.20240803.0400.bz2", "rib.20240803.0600.bz2", "rib.20240803.0800.bz2", "rib.20240803.1000.bz2", "rib.20240803.1200.bz2", "rib.20240803.1400.bz2", "rib.20240803.1600.bz2", "rib.20240803.1800.bz2", "rib.20240803.2000.bz2", "rib.20240803.2200.bz2", "rib.20240804.0000.bz2", "rib.20240804.0200.bz2", "rib.20240804.0400.bz2", "rib.20240804.0600.bz2", "rib.20240804.0800.bz2", "rib.20240804.1000.bz2", "rib.20240804.1200.bz2", "rib.20240804.1400.bz2", "rib.20240804.1600.bz2", "rib.20240804.1800.bz2", "rib.20240804.2000.bz2", "rib.20240804.2200.bz2", "rib.20240805.0000.bz2", "rib.20240805.0200.bz2", "rib.20240805.0400.bz2", "rib.20240805.0600.bz2", "rib.20240805.0800.bz2", "rib.20240805.1000.bz2", "rib.20240805.1200.bz2", "rib.20240805.1400.bz2", "rib.20240805.1600.bz2", "rib.20240805.1800.bz2", "rib.20240805.2000.bz2", "rib.20240805.2200.bz2", "rib.20240806.0000.bz2", "rib.20240806.0200.bz2", "rib.20240806.0400.bz2", "rib.20240806.0600.bz2", "rib.20240806.0800.bz2", "rib.20240806.1000.bz2", "rib.20240806.1200.bz2", "rib.20240806.1400.bz2", "rib.20240806.1600.bz2", "rib.20240806.1800.bz2", "rib.20240806.2000.bz2", "rib.20240806.2200.bz2", "rib.20240807.0000.bz2", "rib.20240807.0200.bz2", "rib.20240807.0400.bz2", "rib.20240807.0600.bz2", "rib.20240807.0800.bz2", "rib.20240807.1000.bz2", "rib.20240807.1200.bz2", "rib.20240807.1400.bz2", "rib.20240807.1600.bz2", "rib.20240807.1800.bz2", "rib.20240807.2000.bz2", "rib.20240807.2200.bz2", "rib.20240808.0000.bz2", "rib.20240808.0200.bz2", "rib.20240808.0400.bz2", "rib.20240808.0600.bz2", "rib.20240808.0800.bz2", "rib.20240808.1000.bz2", "rib.20240808.1200.bz2", "rib.20240808.1400.bz2"];
    let j = ["rib.20240701.0000.bz2", "rib.20240701.0200.bz2", "rib.20240701.0400.bz2", "rib.20240701.0600.bz2", "rib.20240701.0800.bz2", "rib.20240701.1000.bz2", "rib.20240701.1200.bz2", "rib.20240701.1400.bz2", "rib.20240701.1600.bz2", "rib.20240701.1800.bz2", "rib.20240701.2000.bz2", "rib.20240701.2045.bz2", "rib.20240701.2200.bz2", "rib.20240702.0000.bz2", "rib.20240702.0200.bz2", "rib.20240702.0400.bz2", "rib.20240702.0600.bz2", "rib.20240702.0800.bz2", "rib.20240702.1000.bz2", "rib.20240702.1200.bz2", "rib.20240702.1400.bz2", "rib.20240702.1600.bz2", "rib.20240702.1800.bz2", "rib.20240702.2000.bz2", "rib.20240702.2200.bz2", "rib.20240703.0000.bz2", "rib.20240703.0200.bz2", "rib.20240703.0400.bz2", "rib.20240703.0600.bz2", "rib.20240703.0800.bz2", "rib.20240703.1000.bz2", "rib.20240703.1200.bz2", "rib.20240703.1400.bz2", "rib.20240703.1600.bz2", "rib.20240703.1800.bz2", "rib.20240703.2000.bz2", "rib.20240703.2200.bz2", "rib.20240704.0000.bz2", "rib.20240704.0200.bz2", "rib.20240704.0400.bz2", "rib.20240704.0600.bz2", "rib.20240704.0800.bz2", "rib.20240704.1000.bz2", "rib.20240704.1200.bz2", "rib.20240704.1400.bz2", "rib.20240704.1600.bz2", "rib.20240704.1800.bz2", "rib.20240704.2000.bz2", "rib.20240704.2200.bz2", "rib.20240705.0000.bz2", "rib.20240705.0200.bz2", "rib.20240705.0400.bz2", "rib.20240705.0600.bz2", "rib.20240705.0800.bz2", "rib.20240705.1000.bz2", "rib.20240705.1124.bz2", "rib.20240705.1200.bz2", "rib.20240705.1400.bz2", "rib.20240705.1600.bz2", "rib.20240705.1800.bz2", "rib.20240705.2000.bz2", "rib.20240705.2200.bz2", "rib.20240706.0000.bz2", "rib.20240706.0200.bz2", "rib.20240706.0400.bz2", "rib.20240706.0600.bz2", "rib.20240706.0800.bz2", "rib.20240706.1000.bz2", "rib.20240706.1200.bz2", "rib.20240706.1400.bz2", "rib.20240706.1600.bz2", "rib.20240706.1800.bz2", "rib.20240706.2000.bz2", "rib.20240706.2200.bz2", "rib.20240707.0000.bz2", "rib.20240707.0200.bz2", "rib.20240707.0400.bz2", "rib.20240707.0600.bz2", "rib.20240707.0800.bz2", "rib.20240707.1000.bz2", "rib.20240707.1200.bz2", "rib.20240707.1400.bz2", "rib.20240707.1600.bz2", "rib.20240707.1800.bz2", "rib.20240707.2000.bz2", "rib.20240707.2200.bz2", "rib.20240708.0000.bz2", "rib.20240708.0200.bz2", "rib.20240708.0400.bz2", "rib.20240708.0600.bz2", "rib.20240708.0800.bz2", "rib.20240708.1000.bz2", "rib.20240708.1200.bz2", "rib.20240708.1400.bz2", "rib.20240708.1600.bz2", "rib.20240708.1800.bz2", "rib.20240708.2000.bz2", "rib.20240708.2200.bz2", "rib.20240709.0000.bz2", "rib.20240709.0200.bz2", "rib.20240709.0400.bz2", "rib.20240709.0600.bz2", "rib.20240709.0800.bz2", "rib.20240709.1000.bz2", "rib.20240709.1200.bz2", "rib.20240709.1400.bz2", "rib.20240709.1600.bz2", "rib.20240709.1800.bz2", "rib.20240709.2000.bz2", "rib.20240709.2200.bz2", "rib.20240710.0000.bz2", "rib.20240710.0200.bz2", "rib.20240710.0400.bz2", "rib.20240710.0600.bz2", "rib.20240710.0800.bz2", "rib.20240710.1000.bz2", "rib.20240710.1200.bz2", "rib.20240710.1400.bz2", "rib.20240710.1600.bz2", "rib.20240710.1800.bz2", "rib.20240710.2000.bz2", "rib.20240710.2200.bz2", "rib.20240711.0000.bz2", "rib.20240711.0200.bz2", "rib.20240711.0400.bz2", "rib.20240711.0600.bz2", "rib.20240711.0800.bz2", "rib.20240711.1000.bz2", "rib.20240711.1200.bz2", "rib.20240711.1400.bz2", "rib.20240711.1600.bz2", "rib.20240711.1800.bz2", "rib.20240711.2000.bz2", "rib.20240711.2200.bz2", "rib.20240712.0000.bz2", "rib.20240712.0200.bz2", "rib.20240712.0400.bz2", "rib.20240712.0600.bz2", "rib.20240712.0800.bz2", "rib.20240712.1000.bz2", "rib.20240712.1200.bz2", "rib.20240712.1400.bz2", "rib.20240712.1600.bz2", "rib.20240712.1800.bz2", "rib.20240712.2000.bz2", "rib.20240712.2200.bz2", "rib.20240713.0000.bz2", "rib.20240713.0200.bz2", "rib.20240713.0400.bz2", "rib.20240713.0600.bz2", "rib.20240713.0800.bz2", "rib.20240713.1000.bz2", "rib.20240713.1200.bz2", "rib.20240713.1400.bz2", "rib.20240713.1600.bz2", "rib.20240713.1800.bz2", "rib.20240713.2000.bz2", "rib.20240713.2200.bz2", "rib.20240714.0000.bz2", "rib.20240714.0200.bz2", "rib.20240714.0400.bz2", "rib.20240714.0600.bz2", "rib.20240714.0800.bz2", "rib.20240714.1000.bz2", "rib.20240714.1200.bz2", "rib.20240714.1400.bz2", "rib.20240714.1600.bz2", "rib.20240714.1800.bz2", "rib.20240714.2000.bz2", "rib.20240714.2200.bz2", "rib.20240715.0000.bz2", "rib.20240715.0200.bz2", "rib.20240715.0400.bz2", "rib.20240715.0600.bz2", "rib.20240715.0800.bz2", "rib.20240715.1000.bz2", "rib.20240715.1200.bz2", "rib.20240715.1400.bz2", "rib.20240715.1600.bz2", "rib.20240715.1800.bz2", "rib.20240715.2000.bz2", "rib.20240715.2200.bz2", "rib.20240716.0000.bz2", "rib.20240716.0200.bz2", "rib.20240716.0400.bz2", "rib.20240716.0600.bz2", "rib.20240716.0800.bz2", "rib.20240716.1000.bz2", "rib.20240716.1200.bz2", "rib.20240716.1400.bz2", "rib.20240716.1600.bz2", "rib.20240716.1800.bz2", "rib.20240716.2000.bz2", "rib.20240716.2200.bz2", "rib.20240717.0000.bz2", "rib.20240717.0200.bz2", "rib.20240717.0400.bz2", "rib.20240717.0600.bz2", "rib.20240717.0800.bz2", "rib.20240717.1000.bz2", "rib.20240717.1200.bz2", "rib.20240717.1400.bz2", "rib.20240717.1600.bz2", "rib.20240717.1800.bz2", "rib.20240717.2000.bz2", "rib.20240717.2200.bz2", "rib.20240718.0000.bz2", "rib.20240718.0200.bz2", "rib.20240718.0400.bz2", "rib.20240718.0600.bz2", "rib.20240718.0800.bz2", "rib.20240718.1000.bz2", "rib.20240718.1200.bz2", "rib.20240718.1400.bz2", "rib.20240718.1600.bz2", "rib.20240718.1800.bz2", "rib.20240718.2000.bz2", "rib.20240718.2200.bz2", "rib.20240719.0000.bz2", "rib.20240719.0200.bz2", "rib.20240719.0400.bz2", "rib.20240719.0600.bz2", "rib.20240719.0800.bz2", "rib.20240719.1000.bz2", "rib.20240719.1200.bz2", "rib.20240719.1400.bz2", "rib.20240719.1600.bz2", "rib.20240719.1800.bz2", "rib.20240719.2000.bz2", "rib.20240719.2200.bz2", "rib.20240720.0000.bz2", "rib.20240720.0200.bz2", "rib.20240720.0400.bz2", "rib.20240720.0600.bz2", "rib.20240720.0800.bz2", "rib.20240720.1000.bz2", "rib.20240720.1200.bz2", "rib.20240720.1400.bz2", "rib.20240720.1600.bz2", "rib.20240720.1800.bz2", "rib.20240720.2000.bz2", "rib.20240720.2200.bz2", "rib.20240721.0000.bz2", "rib.20240721.0200.bz2", "rib.20240721.0400.bz2", "rib.20240721.0600.bz2", "rib.20240721.0800.bz2", "rib.20240721.1000.bz2", "rib.20240721.1200.bz2", "rib.20240721.1400.bz2", "rib.20240721.1600.bz2", "rib.20240721.1800.bz2", "rib.20240721.2000.bz2", "rib.20240721.2200.bz2", "rib.20240722.0000.bz2", "rib.20240722.0200.bz2", "rib.20240722.0400.bz2", "rib.20240722.0600.bz2", "rib.20240722.0800.bz2", "rib.20240722.1000.bz2", "rib.20240722.1200.bz2", "rib.20240722.1400.bz2", "rib.20240722.1600.bz2", "rib.20240722.1800.bz2", "rib.20240722.2000.bz2", "rib.20240722.2200.bz2", "rib.20240723.0000.bz2", "rib.20240723.0200.bz2", "rib.20240723.0400.bz2", "rib.20240723.0600.bz2", "rib.20240723.0800.bz2", "rib.20240723.1000.bz2", "rib.20240723.1200.bz2", "rib.20240723.1400.bz2", "rib.20240723.1600.bz2", "rib.20240723.1800.bz2", "rib.20240723.2000.bz2", "rib.20240723.2200.bz2", "rib.20240724.0000.bz2", "rib.20240724.0200.bz2", "rib.20240724.0400.bz2", "rib.20240724.0600.bz2", "rib.20240724.0800.bz2", "rib.20240724.1000.bz2", "rib.20240724.1200.bz2", "rib.20240724.1400.bz2", "rib.20240724.1600.bz2", "rib.20240724.1800.bz2", "rib.20240724.2000.bz2", "rib.20240724.2200.bz2", "rib.20240725.0000.bz2", "rib.20240725.0200.bz2", "rib.20240725.0400.bz2", "rib.20240725.0600.bz2", "rib.20240725.0800.bz2", "rib.20240725.1000.bz2", "rib.20240725.1200.bz2", "rib.20240725.1400.bz2", "rib.20240725.1600.bz2", "rib.20240725.1800.bz2", "rib.20240725.2000.bz2", "rib.20240725.2200.bz2", "rib.20240726.0000.bz2", "rib.20240726.0200.bz2", "rib.20240726.0400.bz2", "rib.20240726.0600.bz2", "rib.20240726.0800.bz2", "rib.20240726.1000.bz2", "rib.20240726.1200.bz2", "rib.20240726.1400.bz2", "rib.20240726.1600.bz2", "rib.20240726.1800.bz2", "rib.20240726.2000.bz2", "rib.20240726.2200.bz2", "rib.20240727.0000.bz2", "rib.20240727.0200.bz2", "rib.20240727.0400.bz2", "rib.20240727.0600.bz2", "rib.20240727.0800.bz2", "rib.20240727.1000.bz2", "rib.20240727.1200.bz2", "rib.20240727.1400.bz2", "rib.20240727.1449.bz2", "rib.20240727.1600.bz2", "rib.20240727.1800.bz2", "rib.20240727.2000.bz2", "rib.20240727.2200.bz2", "rib.20240728.0000.bz2", "rib.20240728.0200.bz2", "rib.20240728.0400.bz2", "rib.20240728.0600.bz2", "rib.20240728.0800.bz2", "rib.20240728.1000.bz2", "rib.20240728.1200.bz2", "rib.20240728.1400.bz2", "rib.20240728.1600.bz2", "rib.20240728.1800.bz2", "rib.20240728.2000.bz2", "rib.20240728.2200.bz2", "rib.20240729.0000.bz2", "rib.20240729.0200.bz2", "rib.20240729.0400.bz2", "rib.20240729.0600.bz2", "rib.20240729.0800.bz2", "rib.20240729.1000.bz2", "rib.20240729.1200.bz2", "rib.20240729.1400.bz2", "rib.20240729.1600.bz2", "rib.20240729.1800.bz2", "rib.20240729.2000.bz2", "rib.20240729.2200.bz2", "rib.20240730.0000.bz2", "rib.20240730.0200.bz2", "rib.20240730.0400.bz2", "rib.20240730.0600.bz2", "rib.20240730.0800.bz2", "rib.20240730.1000.bz2", "rib.20240730.1200.bz2", "rib.20240730.1400.bz2", "rib.20240730.1600.bz2", "rib.20240730.1800.bz2", "rib.20240730.2000.bz2", "rib.20240730.2200.bz2", "rib.20240731.0000.bz2", "rib.20240731.0200.bz2", "rib.20240731.0400.bz2", "rib.20240731.0600.bz2", "rib.20240731.0800.bz2", "rib.20240731.1000.bz2", "rib.20240731.1200.bz2", "rib.20240731.1400.bz2", "rib.20240731.1600.bz2", "rib.20240731.1800.bz2", "rib.20240731.2000.bz2", "rib.20240731.2200.bz2"];

    for filename in a {
        let url = format!("{}{}", base_url_rib_august, filename);
        for elem in BgpkitParser::new(&url).unwrap() {
            if let Some(otc) = elem.only_to_customer {
                otc_count += 1;
                total_otc_count += 1;
            } else {
                no_otc_count += 1;
                total_no_otc_count += 1;
            }
        }
        iteration += 1;
        println!("Done with {}/{} iterations. ", iteration, a.len());
        otc_count = 0;
        no_otc_count = 0;
    }

    println!("Elements with OTC: {}; No OTC: {}", total_otc_count, total_no_otc_count);
    let total_count = total_otc_count + total_no_otc_count;
    let total_otc_percentage = (total_otc_count as f64 / total_count as f64) * 100.0;
    println!("Percentage: {:.2}%", total_otc_percentage);
    println!("SSwitching to July.");
    total_otc_count = 0; total_otc_count = 0; otc_count = 0; no_otc_count = 0;
    iteration = 0;
    for filename in j {
        let url = format!("{}{}", base_url_rib_july, filename);
        for elem in BgpkitParser::new(&url).unwrap() {
            if let Some(otc) = elem.only_to_customer {
                otc_count += 1;
                total_otc_count += 1;
            } else {
                no_otc_count += 1;
                total_no_otc_count += 1;
            }
        }
        iteration += 1;
        println!("Done with {}/{} iterations. ", iteration, j.len());
        otc_count = 0;
        no_otc_count = 0;
    }

    println!("Elements with OTC: {}; No OTC: {}", total_otc_count, total_no_otc_count);
    let total_count = total_otc_count + total_no_otc_count;
    let total_otc_percentage = (total_otc_count as f64 / total_count as f64) * 100.0;
    println!("Percentage: {:.2}%", total_otc_percentage);
}
