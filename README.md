# rawsl

### rust version of [awsl](https://github.com/rikaaa0928/awsl)  

## 参数定义  
* 全链路地址  
    + 起点-startAddr  
    + 终点-endAddr
* 单次请求地址  
    + 起点-srcAddr  
    + 终点-dstAddr  
## 主要接口  
```
Server {
    Listen() -> Listener
}
```
```
Listener {
    Accept() -> Connection
    Close()  
}
```
```
Connection {
    标准net.Connection
    EndAddr()
}
```
```
Dialer {
    Dial(Addr) -> Connection
}
```
```
Router {
    Route(Context, Addr) -> Context
}
```