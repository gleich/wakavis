package main

import (
	"fmt"

	"github.com/Matt-Gleich/wakavis/pkg/api"
)

func main() {
	key := api.ReadLocalToken()
	fmt.Println(key)
}
