use std::{io::{self, Error, Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, time::Duration, vec};

use serde::{Deserialize, Serialize};

pub struct Client {
   pub stream: TcpStream,
   message: Vec<u8>
}

pub struct ControlServer {
   server: TcpListener,
   clients: Vec<Client>
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerError {
   status: String,
   reason: String
}

impl ControlServer {
   pub fn start_control_server(address: &String) -> Result<ControlServer, Error> {
      let bind_result = TcpListener::bind(address);

      match bind_result {
         Ok(server) => {
            if let Err(error) = server.set_nonblocking(true) {
               return Err(error);
            }

            Ok(ControlServer { server: server, clients: vec![] })
         },
         Err(error) => {
            Err(error)
         }
      }
   }

   pub fn accept_client(&mut self) {
      match self.server.accept() {
         Ok(client) => {
            if let Ok(()) = client.0.set_read_timeout(Some(Duration::from_millis(20))) {
               self.clients.push(Client::new(client));
            } else {
            }
         },
         Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
         },
         Err(_) => {
         }
      }
   }

   pub fn read_client_message(&mut self) -> Option<(String, &mut Client)> {
      let mut buffer = [0 as u8; 1024];

      for client in self.clients.iter_mut() {
         match client.stream.read(&mut buffer) {
            Ok(read) => {
               if let Some(index) = buffer[0..read].iter().position(|c| *c == 0) {
                  client.message.extend_from_slice(&buffer[0..index]);

                  if let Ok(message) = std::str::from_utf8(&client.message.clone()) {
                     client.message.clear();

                     return Some((message.to_string(), client));
                  } else {
                     client.message.clear();
                  }
               } else {
                  client.message.extend_from_slice(&buffer[0..read]);
               }
            },
            Err(error) if error.kind() == io::ErrorKind::WouldBlock || error.kind() == io::ErrorKind::TimedOut => {

            },
            Err(_) => {

            }
         }
      }

      None
   }
}

pub fn send_error_message_to_client(message: String, client: &mut Client) {
   let error_message = ServerError {
      status: "fail".to_string(),
      reason: message
   };

   let json = serde_json::to_string(&error_message).unwrap_or("".to_string());

   let mut buffer: Vec<u8> = Vec::new();

   buffer.extend_from_slice(json.as_bytes());
   buffer.push(0);

   client.stream.write_all(&buffer).unwrap_or(());
}

impl Client {
   fn new(client: (TcpStream, SocketAddr)) -> Client {
      return Client { stream: client.0, message: vec![] };
   }
}