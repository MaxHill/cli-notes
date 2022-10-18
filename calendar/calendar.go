package calendar

import (
	"fmt"
	"github.com/apognu/gocal"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type Event struct {
	Summary   string
	Details   string
	Start     time.Time
	End       time.Time
	Location  string
	Status    string
	Attendees []string
}

func (e Event) stringAttendees() string {
	return strings.Join(e.Attendees, ", ")
}

func (e Event) stringTime() string {
	return fmt.Sprintf("%s-%s", e.Start.Format("15:00"), e.End.Format("15:00"))
}

// implement the list.Item interface
func (e Event) FilterValue() string {
	return e.Summary
}

func (e Event) Title() string {
	return e.Summary
}

func (e Event) Description() string {
	return e.stringTime()
}

func fromGocal(event gocal.Event) Event {
	var attendees []string
	for _, attendee := range event.Attendees {
		a := fmt.Sprintf("%s, ", strings.ReplaceAll(attendee.Cn, `"`, ""))
		attendees = append(attendees, a)
	}
	return Event{
		Summary:   event.Summary,
		Details:   event.Description,
		Start:     *event.Start,
		End:       *event.End,
		Location:  event.Location,
		Status:    event.Status,
		Attendees: attendees,
	}
}

func GetAllEvents(root string) []Event {
	var events []Event

	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			fmt.Println(err)
			return nil
		}

		if !info.IsDir() && filepath.Ext(path) == ".ics" {
			f, _ := os.Open(path)
			defer f.Close()

			c := gocal.NewParser(f)
			c.Parse()

			for _, event := range c.Events {
				if hasStartAndEnd(event) {
					events = append(events, fromGocal(event))
				}
			}
		}

		return nil
	})

	if err != nil {
		log.Fatal(err)
	}

	return events

}

func hasStartAndEnd(event gocal.Event) bool {
	return event.Start != nil && event.End != nil
}

func ActiveEvents(events []Event) []Event {
	var filtered []Event

	for _, event := range events {
		if event.Start.Before(time.Now()) && event.End.After(time.Now()) {
			filtered = append(filtered, event)
		}
	}

	return filtered
}

func (event Event) ToString() string {
	return fmt.Sprintf(`---
title: %s
attendees: %s
time: %s
---

---
notes: %s`, event.Summary, event.stringAttendees(), event.stringTime(), event.Details)
}
