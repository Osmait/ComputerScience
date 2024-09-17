package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
)

func main() {
	conn, err := net.Dial("tcp", "127.0.0.1:8000")
	if err != nil {
		panic(err)
	}
	defer conn.Close()
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter Messages: ")
	messages, _ := reader.ReadString('\n')
	fmt.Fprint(conn, messages)
	response, _ := bufio.NewReader(conn).ReadString('\n')
	fmt.Print("server Response: " + response)
}
