package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"
)

func getCalendarFiles(root string) []string {
	var files []string

	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			fmt.Println(err)
			return nil
		}

		if !info.IsDir() && filepath.Ext(path) == ".ics" {
			dat, err := os.ReadFile(path)
			if err != nil {
				log.Fatal(err)
			}

			files = append(files, string(dat))
		}

		return nil
	})

	if err != nil {
		log.Fatal(err)
	}

	return files

}

func main() {

	root := "/Users/maxhill/Library/Calendars"

	files := getCalendarFiles(root)

	for _, file := range files {
		fmt.Println(file)
	}
}
