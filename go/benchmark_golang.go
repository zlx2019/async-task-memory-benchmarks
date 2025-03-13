package main

import (
	"fmt"
	"os"
	"strconv"
	"sync"
	"time"
)

func main() {
	taskNums, err := strconv.Atoi(os.Args[1])
	if err != nil {
		panic(err)
	}
	fmt.Printf("startup: %d task. \n", taskNums)
	var wg sync.WaitGroup
	for i := 0; i < taskNums; i++ {
		wg.Add(1)
		go func() {
			time.Sleep(time.Second * 10)
			wg.Done()
		}()
	}
	wg.Wait()
	fmt.Println("shutdown. \n")
}
