package main

import (
	"aoc-2020-kocurek/src/utils"
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Range struct {
	start int
	end   int
}

type Word struct {
	name   string
	ranges []Range
}

type Tickets struct {
	words         []Word
	myTicket      []int
	nearbyTickets [][]int
}

const resultWord = "departure"

func main() {
	lines, err := utils.ReadInput()
	if err != nil {
		log.Fatal(err)
		return
	}
	tickets, err := parseInput(lines)
	if err != nil {
		log.Fatal(err)
		return
	}

	tickets.nearbyTickets = filteredInvalidTickets(&tickets)
	wordToField, err := getWordToField(&tickets)
	if err != nil {
		log.Fatal(err)
		return
	}
	result := getResult(&tickets, wordToField)

	fmt.Println(result)
}

func parseInput(lines []string) (Tickets, error) {
	var lineRanges []Word
	var myTicket []int
	var nearbyTickets [][]int

	i := 0
	for lines[i] != "" {
		parts := strings.Split(lines[i], ": ") // class: 0-1 or 4-19

		name := parts[0]                             // class
		rangesStr := strings.Split(parts[1], " or ") // 0-1 or 4-19

		var ranges []Range
		for _, single_range := range rangesStr {
			nums := strings.Split(single_range, "-") // 0, 1

			start, err := strconv.Atoi(nums[0])
			if err != nil {
				return Tickets{}, err
			}
			end, err := strconv.Atoi(nums[1])
			if err != nil {
				return Tickets{}, err
			}

			ranges = append(ranges, Range{start, end})
		}

		lineRanges = append(lineRanges, Word{name, ranges})
		i++
	}

	i += 2 // empty line and "your ticket:"
	myTicket, err := parseTicket(lines[i])
	if err != nil {
		return Tickets{}, err
	}
	i++

	i += 2 // empty line and "nearby tickets:"
	for i < len(lines) {
		nearbyTicket, err := parseTicket(lines[i])
		if err != nil {
			return Tickets{}, err
		}
		nearbyTickets = append(nearbyTickets, nearbyTicket)
		i++
	}

	return Tickets{lineRanges, myTicket, nearbyTickets}, nil
}

func parseTicket(line string) ([]int, error) {
	nums := strings.Split(line, ",")
	var ticket []int
	for _, num := range nums {
		val, err := strconv.Atoi(num)
		if err != nil {
			return nil, err
		}
		ticket = append(ticket, val)
	}
	return ticket, nil
}

func filteredInvalidTickets(tickets *Tickets) [][]int {
	var validTickets [][]int
	var allRanges []Range

	for _, word := range tickets.words {
		allRanges = append(allRanges, word.ranges...)
	}

	for _, ticket := range tickets.nearbyTickets {
		if ticketFitsWordRanges(ticket, allRanges) {
			validTickets = append(validTickets, ticket)
		}
	}

	return validTickets
}

func ticketFitsWordRanges(ticket []int, ranges []Range) bool {
	for _, num := range ticket {
		if !numFitsOneOfWordRanges(num, ranges) {
			return false
		}
	}
	return true
}

func getWordToField(tickets *Tickets) ([]int, error) {
	// Precompute, so that we don't have to redo this calculation
	wordMatchesField := getWordMatchesField(tickets)

	wordToField := make([]int, len(tickets.words))
	fieldUsed := make([]bool, len(tickets.words))

	if !canMatchWordsToFields(0, wordToField, wordMatchesField, fieldUsed, tickets) {
		return nil, fmt.Errorf("no valid field to word mapping found")
	}

	return wordToField, nil
}

func getWordMatchesField(tickets *Tickets) map[int][]int {
	// Inverting is not necessary, but might give us better cache locality
	invertedTickets := getInvertedTickets(tickets)

	wordMatchesField := make(map[int][]int)
	for wordIdx, word := range tickets.words {
		for fieldIdx, field := range invertedTickets {
			if allFieldNumsMatchWordRanges(field, word.ranges) {
				wordMatchesField[wordIdx] = append(wordMatchesField[wordIdx], fieldIdx)
			}
		}
	}
	return wordMatchesField
}

func getInvertedTickets(tickets *Tickets) [][]int {
	invertedTickets := make([][]int, len(tickets.myTicket))

	for _, ticket := range tickets.nearbyTickets {
		for i, num := range ticket {
			invertedTickets[i] = append(invertedTickets[i], num)
		}
	}

	for i, num := range tickets.myTicket {
		invertedTickets[i] = append(invertedTickets[i], num)
	}

	return invertedTickets
}

func canMatchWordsToFields(wordIdx int, wordToField []int, wordMatchesField map[int][]int, fieldUsed []bool, tickets *Tickets) bool {
	if wordIdx == len(tickets.words) {
		return true
	}

	for _, fieldIdx := range wordMatchesField[wordIdx] {
		if fieldUsed[fieldIdx] {
			continue
		}

		wordToField[wordIdx] = fieldIdx
		fieldUsed[fieldIdx] = true

		if canMatchWordsToFields(wordIdx+1, wordToField, wordMatchesField, fieldUsed, tickets) {
			return true
		}

		fieldUsed[fieldIdx] = false
	}

	return false
}

func allFieldNumsMatchWordRanges(nums []int, ranges []Range) bool {
	for _, num := range nums {
		if !numFitsOneOfWordRanges(num, ranges) {
			return false
		}
	}
	return true
}

func numFitsOneOfWordRanges(num int, ranges []Range) bool {
	for _, r := range ranges {
		if r.start <= num && num <= r.end {
			return true
		}
	}
	return false
}

func getResult(tickets *Tickets, wordToField []int) int64 {
	result := int64(1)

	for wordIdx, fieldIdx := range wordToField {
		if strings.HasPrefix(tickets.words[wordIdx].name, resultWord) {
			result *= int64(tickets.myTicket[fieldIdx])
		}
	}

	return result
}
