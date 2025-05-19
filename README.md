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




</details>