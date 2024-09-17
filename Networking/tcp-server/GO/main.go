package main

import (
	"bufio"
	"fmt"
	"net"
)

func handleConnection(conn net.Conn) {
	defer conn.Close()
	scanner := bufio.NewScanner(conn)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
		fmt.Fprintf(conn, "Recived: %s\n", scanner.Text())
	}
}

func main() {
	listenner, err := net.Listen("tcp", ":8000")
	if err != nil {
		panic(err)
	}
	defer listenner.Close()
	fmt.Println("server listening on port 8000")

	for {
		conn, err := listenner.Accept()
		if err != nil {
			fmt.Println(err)
			continue
		}
		fmt.Printf("Accepted connection from: %s\n", conn.RemoteAddr())
		go handleConnection(conn)
	}
}
