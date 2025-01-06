# **ITM Parser**

`itm-parser` is a command-line tool for parsing and formatting ITM (Instrumentation Trace Macrocell) output. It simplifies debugging by converting raw ITM messages into human-readable ASCII text, working seamlessly with `probe-rs` in the background.

> **Note:** `itm-parser` internally uses `probe-rs`. Ensure `probe-rs` is installed before using this tool.

---

## **Features**
- Parses ITM Instrumentation packets (e.g., `Ok(Instrumentation { port: 0, payload: [...] })`).
- Decodes payloads into readable ASCII text.
- Supports newline characters (`\n`) in payloads for proper log message formatting.
- Fully integrates with `probe-rs` for ITM communication.

---

## **Installation**

### **Step 1: Install `probe-rs`**
`itm-parser` relies on `probe-rs` to interact with your embedded device. Install `probe-rs` using Cargo:

```bash
cargo install probe-rs
```
### **Step 2: Install `itm-parser`**
Once `probe-rs` is installed, install `itm-parser`:

```bash
cargo install itm-parser
```
---

## **Usage**
- Run the `itm-parser` command with required arguments. Internally, it uses probe-rs itm swo to interact with your embedded device.

### **Command Syntax**
```bash
itm-parser --chip <CHIP> [--probe <VID:PID>] <DURATION> <CLOCK> <BAUD>
```
### **Arguments**
1) --chip <CHIP> (required): The target chip identifier (e.g., STM32F303CC).
2) --probe <VID:PID> (optional): The VID:PID of the debug probe to use (e.g., 0483:374b). If omitted, the default probe will be used.
3) <DURATION> (required): Duration of the trace in milliseconds.
4) <CLOCK> (required): Clock speed feeding the TPIU/SWO module in Hz.
5) <BAUD> (required): Desired baud rate for SWO output.

## **Example**
```bash
itm-parser --chip STM32F303CC 10000 8000000 1000000
```

## **Output**
If the raw ITM packets contain:
```
Ok(Instrumentation { port: 0, payload: [72, 101, 108, 108, 111, 10] })
Ok(Instrumentation { port: 0, payload: [87, 111, 114, 108, 100, 10] })
```
`itm-parser` will decode and display:
```
Hello
World
```
