# Tutorial 10 Asynchronous Programming

Name: Belva Ghani Abhinaya

Class: Advance Programming B

Student Number: 2306203526

<details open>
<summary><b>Reflection on Module 10</b></summary>
<br>

### Reflection on Module 10b

#### 1. Experiment 2.1: Original code, and how it runs

- How to run

  1. **Start the server**  
     In **Terminal 1**:
     ```bash
     cargo run --bin server
     ```
  2. **Start three clients**
     In **Terminal 2/3/4** (each in its own tab or window):
     ```bash
     cargo run --bin client
     ```
  3. **Type messages**
     In any client, type a line (e.g. tes) and press Enter.
     Observe that all three clients immediately print:
     ```bash
     [127.0.0.1:XXXXX] hello from A
     ```
     (where XXXXX is the ephemeral port of the sender).

- What Happens
  - The server logs each new connection:
    ```bash
    New connection from 127.0.0.1:XXXXX
    ```
  - Sending a line from one client broadcasts it to every connected client.
  - This confirms our tokio::select! + broadcast‐channel design: one branch reads stdin→sends, the other branch receives broadcasts→prints.

- Screenshots:
  ![Client 1](https://github.com/user-attachments/assets/79121f5e-0204-476a-bc99-48e78028ddac)
  ![Client 2](https://github.com/user-attachments/assets/a3d797bf-d964-4ebe-8afc-ee78d1ea0218)
  ![Client 3](https://github.com/user-attachments/assets/2ab3bdec-c9ef-432d-bbf5-ebab342e8477)

#### 2. Experiment 2.2: Modifying port
**Modifications**
- **Server** (`src/bin/server.rs`): changed
  ```rust
  let listener = tokio::net::TcpListener::bind("127.0.0.1:2000").await?;
  ```
  to
  ```rust
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
  ```
- **Client** (`src/bin/client.rs`): changed
  ```rust
  let (ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
  ```
  to
  ```rust
  let (ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
  ```
- We’re still using the same ws:// scheme—nothing changes in the protocol itself. That scheme is defined in the Uri::from_static("ws://...") call in the client, and the server’s ServerBuilder automatically upgrades incoming TCP connections on that port to WebSockets.

**Results**
![Server](https://github.com/user-attachments/assets/7091b275-7d62-414e-8548-d0add348f550)
![Client 1](https://github.com/user-attachments/assets/3cb4f7c8-ef82-4681-a961-2d5ef992f708)
![Client 2](https://github.com/user-attachments/assets/98d48088-2796-461c-9e1f-027e91c78abc)
![Client 3](https://github.com/user-attachments/assets/aed1266e-3fd0-43d5-b980-3be63d9e6d6f)

#### 3. Experiment 2.3: Small changes, add IP and Port
**Modification**  
- In `client.rs` we extract the `IP:Port` prefix from the incoming text frame (which the server embeds) and print it separately:

  ```rust
  if let Some(txt) = msg.as_text() {
      if let Some((peer, body)) = txt
          .trim_start_matches('[')
          .split_once("] ")
      {
          println!("Client – Msg from {}: {}", peer, body);
      } else {
          println!("Client – {}", txt);
      }
  }
  ```
- In `server.rs` we kept its focus on protocol: receive text frames, log them, and rebroadcast.
- No formatting or parsing of the `IP:Port` prefix—just `ws_stream.send(Message::text(msg))`.

  ```rust
  ws_stream
      .send(Message::text("Welcome to Chat! Type a message"))
      .await?;
  ```
  and
   ```rust
    if let Some(text) = msg.as_text() {
        println!("From client {}: {}", addr, text);
        let framed = format!("[{}] {}", addr, text);
        let _ = bcast_tx.send(framed);
    }`
   ```

**Explanation**
- We keep the server focused purely on protocol (receiving text and rebroadcasting it) and push all presentation logic into the client.
- Parsing the "[IP:Port] message" on the client makes it clear which peer sent each message without cluttering server code with formatting concerns.
- Using split_once("] ") reliably separates the sender prefix from the actual message body in a single operation.
- **Server** stays focused on protocol and observability: greeting new connections and logging raw input.

**Screenshot**
![Server](https://github.com/user-attachments/assets/68c2cb7d-4b4a-49aa-8f82-03b18211e828)
![Client 1](https://github.com/user-attachments/assets/23bc9de3-34e2-43e8-9bd3-d06c1cd4137f)
![Client 2](https://github.com/user-attachments/assets/4540a5fc-c530-45ca-8aa2-a20b204478a0)
![Client 3](https://github.com/user-attachments/assets/b946cf4e-b034-4647-bbf4-5237a9859e35)

</details>