package api

import (
	"io/ioutil"
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReadLocalToken(t *testing.T) {
	homeDir, err := os.UserHomeDir()
	assert.NoError(t, err)

	cfgPath := filepath.Join(homeDir, ".wakatime.cfg")
	_, err = os.Stat(cfgPath)
	cfgExists := !os.IsNotExist(err)

	if cfgExists {
		assert.Contains(t, ReadLocalToken(), "")
	} else {
		tt := []struct {
			cfgContent    string
			expectedToken string
		}{
			{
				cfgContent:    "[settings]\ndebug = false\napi_key = 1234",
				expectedToken: "1234",
			},
			{
				cfgContent:    "api_key = 2ajjgjdk20fjkdjfkd", // Not a real API key
				expectedToken: "2ajjgjdk20fjkdjfkd",
			},
			{
				cfgContent:    "api_key = 2394\ndebug = false",
				expectedToken: "2394",
			},
			{
				cfgContent:    "api_key=2394\ndebug = false",
				expectedToken: "2394",
			},
		}

		for _, test := range tt {
			err = ioutil.WriteFile(cfgPath, []byte(test.cfgContent), 0666)
			assert.NoError(t, err)
			assert.Equal(t, test.expectedToken, ReadLocalToken())
			err = os.Remove(cfgPath)
			assert.NoError(t, err)
		}
	}
}
