extern crate curl;

use network::curl::easy::Easy;

// pub fn simple_get(client: Client, path: String) {
//     let mut handle = Easy::new();
//     let mut buffer = Vec::new();
// }
//

use types::Client;

pub fn simple_get(client: Client, path: &str) -> Result<Vec<u8>, u32> {
    let reqpath = generate_path(client, path);


    let buffer = {
        let mut handle = Easy::new();
        let mut buffer = Vec::new();

        handle.url(&reqpath).unwrap();

        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();

        transfer.perform().unwrap();

        buffer
    };

    Ok(buffer)
}

pub fn generate_path(client: Client, path: &str) -> String {
    let mut buf = String::with_capacity(path.len() + client.socket_path.len());

    buf.push_str(&client.socket_path);
    buf.push('/');
    buf.push_str(path);

    buf
}
