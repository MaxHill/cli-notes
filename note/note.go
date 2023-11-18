package note

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"time"

	cal "github.com/MaxHill/cli-notes/calendar"
)

type Note struct {
	Path     string
	FileName string
	Content  string
}

func (n Note) FullFile() string {
	return fmt.Sprintf("%s.md", filepath.Join(n.Path, n.FileName))
}

func (n Note) NoteExists() bool {
	_, err := os.Stat(n.FullFile())

	if errors.Is(err, os.ErrNotExist) {
		return false
	}

	return true
}

func (n Note) Save() error {
	data := []byte(n.Content)

	if n.NoteExists() {
		return nil
	}

	os.MkdirAll(n.Path, os.ModePerm)
	err := ioutil.WriteFile(n.FullFile(), data, 0644)

	if err != nil {
		return err
	}
	return nil
}

func CreateFilename(start time.Time, end time.Time, name string) string {
	var nonAlphanumericRegex = regexp.MustCompile(`[^a-zA-Z0-9 ]+`)
	layoutStart := "2006-01-02-15:04"
	layoutEnd := "15:04"

	return fmt.Sprintf(
		"%s-%s-%s",
		start.Format(layoutStart),
		end.Format(layoutEnd),
		strings.ReplaceAll(nonAlphanumericRegex.ReplaceAllString(name, ""), " ", "-"),
	)
}

func FromEvent(rootPath string, event cal.Event) Note {

	return Note{
		Path:     rootPath,
		FileName: CreateFilename(event.Start, event.End, event.Summary),
		Content:  event.ToString(),
	}
}
