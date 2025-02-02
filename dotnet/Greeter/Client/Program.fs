open System
open Grpc.Core
open Protocol.ProtoBuf



type HelloServiceImpl(client : HelloService.HelloServiceClient) =
        member _.SayHello(greeting : string) =
            let request = new HelloReq(Greeting = greeting)
            client.SayHello(request)


[<EntryPoint>]
let main argv =
    let channel = new Channel("0.0.0.0:50051", ChannelCredentials.Insecure)
    let client = new HelloServiceImpl(new HelloService.HelloServiceClient(channel))
    
    let result = client.SayHello("Hi~")
    Console.WriteLine(result.Reply)
    0 // return an integer exit code
