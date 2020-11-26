package api

import (
	"context"
	"net/http"

	"github.com/Matt-Gleich/statuser/v2"
	"github.com/YouEclipse/wakatime-go/pkg/wakatime"
)

// Get the data for the current week
func GetWeekData(apiKey string) wakatime.StatsData {
	var (
		client = wakatime.NewClient(apiKey, &http.Client{})
		ctx    = context.Background()
		query  = &wakatime.StatsQuery{}
	)
	stats, err := client.Stats.Current(ctx, wakatime.RangeLast7Days, query)
	if err != nil {
		statuser.Error("Failed to get data for this week", err, 1)
	}
	return stats.Data
}
