use std::env;
use std::process::Command;

#[derive(Debug)]
struct URI {
    scheme: String,
    host: String,
    path: String,
    query: String,
    fragment: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let uri = uri_parser(args);

    match &uri.host[..] {
        "execute" => {
            if uri.path != "" {
                let mut command = Command::new(&uri.path);
                command.status().expect("process failed to execute");
            } else {
                println!("No command provided");
            }
        }
        _ => {}
    }
}

fn uri_parser(args: Vec<String>) -> URI {
    let uri = URI {
        scheme: "".to_string(),
        host: "".to_string(),
        path: "".to_string(),
        query: "".to_string(),
        fragment: "".to_string(),
    };

    if args.len() != 1 {
        let uri_ref: Vec<&str> = args[1].split("://").collect();
        if uri_ref.len() == 2 {
            let host_ref: Vec<&str> = uri_ref[1].split("/").collect();
            if host_ref.len() == 2 {
                let path_ref: Vec<&str> = host_ref[1].split("?").collect();
                if path_ref.len() == 2 {
                    let query_ref: Vec<&str> = path_ref[1].split("#").collect();
                    if query_ref.len() == 2 {
                        let uri = URI {
                            scheme: uri_ref[0].to_string(),
                            host: host_ref[0].to_string(),
                            path: path_ref[0].to_string(),
                            query: query_ref[0].to_string(),
                            fragment: query_ref[1].to_string(),
                        };
                        uri
                    } else {
                        let uri = URI {
                            scheme: uri_ref[0].to_string(),
                            host: host_ref[0].to_string(),
                            path: path_ref[0].to_string(),
                            query: query_ref[0].to_string(),
                            fragment: "".to_string(),
                        };
                        uri
                    }
                } else {
                    let uri = URI {
                        scheme: uri_ref[0].to_string(),
                        host: host_ref[0].to_string(),
                        path: path_ref[0].to_string(),
                        query: "".to_string(),
                        fragment: "".to_string(),
                    };
                    uri
                }
            } else {
                let uri = URI {
                    scheme: uri_ref[0].to_string(),
                    host: host_ref[0].to_string(),
                    path: "".to_string(),
                    query: "".to_string(),
                    fragment: "".to_string(),
                };
                uri
            }
        } else {
            println!("Malformed Universal Resource Indicator");
            uri
        }
    } else {
        println!("No Universal Resource Indicator provided");
        uri
    }
}
