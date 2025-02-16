
# Pingora Proxy App

A rust based proxy server created using Pingora Framework


Read more about [Pingora](https://blog.cloudflare.com/pingora-open-source/)

Pingora github [repo](https://github.com/cloudflare/pingora)

#### NOTE !! 
#### Pingora is built primarily for Linux. This app may not work properly on windows even after Perl installation (from personal experience)


## Installation steps

- Install Rust [Follow the steps](https://www.rust-lang.org/tools/install)
- clone this repo
``` 
git clone <Repo URL> 
```
- run command
```
Cargo build
```
- run command
```
Cargo run
```
 The App should Run now on port : 6193 for Reverse Proxy  and port : 6194 for Forward Proxy 

# Test The Proxies

## Forward Proxy
#### NOTE :  The proxy is configured for HTTPS !!

run  the following commands to test the Proxy :
```
curl -v -x localhost:6194 example.com
```
or 
```
curl -v -x localhost:6194 http://example.com
```
#### samele output
<img width="1440" alt="413592228-ac47eca0-6cad-4bc8-bb05-b4d7ffe2802c" src="https://github.com/user-attachments/assets/006f7ec9-bb63-4c13-87cd-fd4b6cdf87b0" />


## Reverse Proxy

The Reverse Proxy is configured for [https://jsonplaceholder.typicode.com/](https://jsonplaceholder.typicode.com/)

run  the following commands to test the Proxy :
```
curl -v localhost:6193/todos
```
or 
```
curl -v localhost:6193/todos/1
```
#### sample output
<img width="486" alt="413592237-f8c74781-fab8-43b8-9d34-6fc862643bbd" src="https://github.com/user-attachments/assets/daacae40-14e6-49ea-b7ec-8cb743c51712" />



