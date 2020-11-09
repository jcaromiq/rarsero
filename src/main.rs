use reqwest::blocking::Response;
use rayon::prelude::*;
use std::env;
use colored::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let domain = format!("{}/robots.txt", args.get(1).unwrap());
    let res = reqwest::blocking::get(domain.as_str()).unwrap();
    let paths = content(res).unwrap();
    paths.par_iter().for_each(|url| {
        let result = reqwest::blocking::get(url).unwrap();
        match result.status() {
            reqwest::StatusCode::OK => {
                println!("{} {}", url, "200 OK".green());
            }

            status_code => {
                println!("{} {} {}", url, status_code.as_str().red(), status_code.canonical_reason().unwrap().red());
            }
        }
    });
}

fn content(response: Response) -> Result<Vec<String>, &'static str> {
    if let reqwest::StatusCode::OK = response.status() {
        let host = String::from(response.url().host_str().unwrap());
        let scheme = String::from(response.url().scheme());
        Ok(response.text().unwrap().lines()
            .filter(|line| line.starts_with("Disallow"))
            .map(|line| line.replace("Disallow: ", ""))
            .map(|l| {
                if !l.starts_with("/") {
                    return format!("/{}", l);
                }
                return l;
            })
            .map(|path| format!("{}://{}{}", scheme, host, path))
            .collect())
    } else {
        Err("robots.txt not found")
    }
}
