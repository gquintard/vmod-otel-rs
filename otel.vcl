vcl 4.1;

import otel_rs from "/path/to/libvmod_otel_rs.so";

sub vcl_recv {			otel_rs.log("entering vcl_recv"); }
sub vcl_deliver {		otel_rs.log("entering vcl_deliver"); }
sub vcl_hit {			otel_rs.log("entering vcl_hit"); }
sub vcl_miss {			otel_rs.log("entering vcl_miss"); }
sub vcl_pass {			otel_rs.log("entering vcl_pass"); }
sub vcl_hash {			otel_rs.log("entering vcl_hash"); }
sub vcl_backend_fetch {		otel_rs.log("entering vcl_backend_fetch"); }
sub vcl_backend_response {	otel_rs.log("entering vcl_backend_response"); }
sub vcl_backend_error {		otel_rs.log("entering vcl_backend_error"); }
sub vcl_synth {			otel_rs.log("entering vcl_synth"); }

sub vcl_recv {
	if (req.restarts == 0) {
		set req.http.traceparent = otel_rs.new_req_span(req.http.traceparent);
	}
}

sub vcl_backend_fetch {
	set bereq.http.traceparent = otel_rs.new_bereq_span(bereq.http.traceparent);
}
