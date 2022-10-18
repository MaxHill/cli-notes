package config

import (
	"fmt"
	"io/ioutil"
	"log"

	"github.com/adrg/xdg"
	"gopkg.in/yaml.v3"
)

type Config struct {
	CalendarRoot string `yaml:"calendarRoot"`
	NotesRootDir string `yaml:"notesRootDir"`
}

func GetConfig() (*Config, error) {
	configFilePath, err := xdg.ConfigFile("cli-note/config.yaml")
	if err != nil {
		log.Fatal(err)
	}

	buf, err := ioutil.ReadFile(configFilePath)
	if err != nil {
		return nil, err
	}

	c := &Config{}
	err = yaml.Unmarshal(buf, c)

	if err != nil {
		return nil, fmt.Errorf("in file %q: %w", configFilePath, err)
	}

	if c.CalendarRoot == "" {
		log.Fatalf("\nSpecify the 'calendarRoot' property in \n%s", configFilePath)
	}

	if c.NotesRootDir == "" {
		log.Fatalf("\nSpecify the 'notesRootDir' property in \n%s", configFilePath)
	}

	return c, err

}
