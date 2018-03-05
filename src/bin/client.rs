extern crate grpc;
extern crate grpc_example;
extern crate protobuf;

use std::thread;

use grpc_example::foobar_grpc::*;
use grpc_example::foobar::*;

fn main() {
    //create a client to talk to a fiven serfer
    let client = FooBarServiceClient::new_plain("34.215.8.182", 80, Default::default()).unwrap();

    let mut req = CabLocationRequest::new();
    req.set_name("foo".to_string());

    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    req.set_location(location);

    //fist RPC call
    let resp = client.record_cab_location(grpc::RequestOptions::new(), req);
    match resp.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_, r, _)) => println!("{:?}", r),
    }

    let mut nearby_req = GetCabRequest::new();
    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    nearby_req.set_location(location);

    //fist RPC call
    let nearby_resp = client.get_cabs(grpc::RequestOptions::new(), nearby_req);
    match nearby_resp.wait() {
        Err(e) => panic!("{:?}", e),
        Ok((_, cab, _)) => println!("{:?}", cab),
    }
}
