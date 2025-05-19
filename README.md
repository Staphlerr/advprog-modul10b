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


</details>