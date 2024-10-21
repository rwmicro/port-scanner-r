<center> 
<img alt='Rust Logo' src='https://almablog-media.s3.ap-south-1.amazonaws.com/image1_3548865930.png' height='100' width='200' />
</center>

# Simple Rust CLI port scanner 

This project is a __port scanner__ implemented in __Rust__, designed to perform asynchronous requests for efficient scanning.

## Getting started
You just need to compile the code into a binary via the command: 

```bash
cargo build --release   
```

Once it's finish you can run the app by using: 

```bash
./target/release/port-scanner-r <SPECIFIED_IP> <PORT_START> <PORT_END>
```

#### Parameters
Replace the placeholders with your specific parameters:

- __<SPECIFIED_IP>__: The IP address you want to scan.
- __<PORT_START>__: The starting port of the range you wish to scan.
- __<PORT_END>__: The ending port of the range you wish to scan.

### Example Usage

To scan IP 192.168.1.1 from port 1 to port 1000, you would run:

```bash
./target/release/port-scanner-r 192.168.1.1 1 1000
```
This command will initiate a scan of ports 1 through 1000 on the specified IP address.

## Performance Comparison

Here’s a comparison of the execution time between the **Python** port scanner and my **Rust** implementation for scanning the same IP address and port range.

### Python Script
The Python script was found on GitHub at this address: 
```
https://github.com/ahervias77/portscanner/blob/master/portscanner.py
```
When running the Python script, the command was:

```bash
time python scanner_port.py 172.67.192.108 0 500
[*] Starting TCP port scan on host 172.67.192.108
[+] TCP scan on host 172.67.192.108 complete
python scanner_port.py 172.67.192.108 0 500  0.14s user 0.11s system 4% cpu 5.353 total
```

- **User time**: 0.14 seconds
- **System time**: 0.11 seconds
- **CPU usage**: 4%
- **Total time**: 5.353 seconds

### Rust Script

When executing my Rust port scanner, the command was:

```bash
time ./target/release/port-scanner-r 172.67.192.108 0 500
Port 80 is open
Port 443 is open
./target/release/port-scanner-r 172.67.192.108 0 500  0.05s user 0.09s system 6% cpu 2.102 total
```

- **User time**: 0.05 seconds
- **System time**: 0.09 seconds
- **CPU usage**: 6%
- **Total time**: 2.102 seconds

### Summary

The Rust implementation outperformed the Python script significantly:

- **Total Time**:
  - **Rust**: 2.102 seconds
  - **Python**: 5.353 seconds

- **Efficiency**:
  - The Rust script executed the task nearly **2.5 times faster** than the Python script.

__Happy scanning!__