pub mod aws;
pub mod azure;
pub(crate) mod common;
pub mod enrichment;
pub mod err;
pub mod maxmind;
pub mod o365;
pub mod tasks;

#[cfg(test)]
mod tst {

    use usiem::prelude::{geo_ip::GeoIpDataset, SiemIp};

    use crate::maxmind::{
        download_maxmind_geo_litle2_asn, download_maxmind_geo_litle2_city,
        download_maxmind_geo_litle2_country, extract_zip_db, join_path_files,
        process_maxmind_geo_lite2_csv,
    };

    #[ignore]
    #[test]
    fn should_load_geoip() {
        let now = std::time::Instant::now();
        #[cfg(not(feature = "slow_geoip"))]
        let dataset = GeoIpDataset::new();
        #[cfg(feature = "slow_geoip")]
        let dataset = GeoIpDataset::new("./slow_geo_ip");
        println!("Duration {}", now.elapsed().as_secs_f32());
        let res = dataset
            .get(&SiemIp::from_ip_str("1.0.0.0").unwrap())
            .unwrap();
        println!("{:?}", res);
        let res = dataset
            .get(&SiemIp::from_ip_str("1.0.4.0").unwrap())
            .unwrap();
        println!("{:?}", res);
        let now = std::time::Instant::now();
        for i in 0..1_000_000 {
            let _res = dataset.get(&SiemIp::V4(i));
        }
        println!("Duration {}", now.elapsed().as_secs_f32());
    }
    #[ignore]
    #[tokio::test]
    async fn should_update_geo_ip() {
        let now = std::time::Instant::now();
        let asn_path = download_maxmind_geo_litle2_asn(
            &std::env::var("MAXMIND_API").expect("Should exists var"),
        )
        .await
        .unwrap();
        let city_path = download_maxmind_geo_litle2_city(
            &std::env::var("MAXMIND_API").expect("Should exists var"),
        )
        .await
        .unwrap();
        let country_path = download_maxmind_geo_litle2_country(
            &std::env::var("MAXMIND_API").expect("Should exists var"),
        )
        .await
        .unwrap();
        let city_path = extract_zip_db(&city_path).await.unwrap();
        let country_path = extract_zip_db(&country_path).await.unwrap();
        let asn_path = extract_zip_db(&asn_path).await.unwrap();
        println!("{:?}", city_path);
        println!("{:?}", country_path);
        println!("{:?}", asn_path);
        let new_path = join_path_files(vec![city_path, country_path, asn_path])
            .await
            .unwrap();
        println!("{:?}", new_path);
        #[cfg(not(feature = "slow_geoip"))]
        let res = process_maxmind_geo_lite2_csv("/tmp/geoip_501122574_db", true, "en")
            .await
            .unwrap();
        #[cfg(feature = "slow_geoip")]
        let res = process_maxmind_geo_lite2_csv("/tmp/geoip_501122574_db", true, "en", "./slow_geo_ip")
            .await
            .unwrap();
        println!("Duration {}", now.elapsed().as_secs_f32());
        let _geoip = res.get(&SiemIp::from_ip_str("1.0.0.0").unwrap()).unwrap();
        let now = std::time::Instant::now();
        for i in 0..1_000_000 {
            let _res = res.get(&SiemIp::V4(i));
        }
        println!("Duration {}", now.elapsed().as_secs_f32());
    }
}
