FROM ubuntu:latest
ADD target/release/isholiday /
CMD ["/isholiday"]
