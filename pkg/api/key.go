package api

import (
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"

	"github.com/Matt-Gleich/statuser/v2"
)

// Read the local API token
func ReadLocalToken() string {
	// Getting the user's home dir
	homeDir, err := os.UserHomeDir()
	if err != nil {
		statuser.Error("Failed to get user home directory.", err, 1)
	}

	// Reading from the config file
	cfgPath := filepath.Join(homeDir, ".wakatime.cfg")
	b, err := ioutil.ReadFile(cfgPath)
	if err != nil {
		statuser.Error("Failed to read from global wakatime config file", err, 1)
	}

	// Parsing the config file
	var (
		conf = string(b)
		key  string
	)
	for _, line := range strings.Split(conf, "\n") {
		keyIdentifier := "api_key"
		if strings.HasPrefix(line, keyIdentifier) {
			key = strings.TrimPrefix(line, keyIdentifier)
			key = strings.Trim(key, " ")
			key = strings.Trim(key, "=")
			key = strings.Trim(key, " ")
			break
		}
	}
	if key == "" {
		statuser.ErrorMsg("Failed to get api key from global config file", 1)
	}

	return key
}
