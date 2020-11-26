package main

import (
	"github.com/Matt-Gleich/wakavis/pkg/api"
)

func main() {
	key := api.ReadLocalToken()
	api.GetWeekData(key)
}
