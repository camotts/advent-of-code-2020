package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strings"
)

const (
	newLine      = "\n"
	emptySpace   = "    "
	middleItem   = "├── "
	continueItem = "│   "
	lastItem     = "└── "
)

var (
	holdingRegex = regexp.MustCompile("^(.*) bags? contain (.*).$")
	countRegex   = regexp.MustCompile("^\\d+ (.*)( bag).*$")
	bagList      = make([]string, 0)
)

type Node struct {
	children []*Node
	bagId    int
	name     string
	parent   *Node
}

func (n *Node) FindName(name string, skipFirst bool) *Node {
	if !skipFirst && bagList[n.bagId] == name {
		return n
	}
	for _, node := range n.children {
		item := node.FindName(name, false)
		if item != nil {
			return item
		}
	}
	return nil
}

func (n *Node) Count() int {
	if len(n.children) == 0 {
		return 1
	}
	ret := 0
	for _, node := range n.children {
		ret = ret + node.Count()
	}
	return ret
}

func (n *Node) CountParents() int {
	if n.parent == nil {
		return 1
	}
	return 1 + n.parent.CountParents()
}

func main() {
	file, err := os.Open("day7\\input")
	//file, err := os.Open("testinput")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	items := make([]string, 0)
	bagCh := make(chan string, 1000)
	bagBuilder := make(map[string]*Node)
	bagMap := make(map[string]int)
	for scanner.Scan() {
		text := scanner.Text()
		bagMap[strings.Split(text, " bag")[0]] = len(bagList)
		currLen := len(bagList)
		bagList = append(bagList, strings.Split(text, " bag")[0])
		if !strings.Contains(text, "no other") {
			//not bottom of tree
			items = append(items, text)
		} else {
			bagCh <- strings.Split(text, " bag")[0]
			bagBuilder[strings.Split(text, " bag")[0]] = &Node{bagId: currLen, name: bagList[currLen]}
		}
	}
	for bag := range bagCh {
		found := GetMatching(items, bag)
		if len(found) == 0 {
			continue
		}
		for i := len(found) - 1; i >= 0; i-- {
			if strings.Contains(found[i].str, bag) {
				otherMatches := holdingRegex.FindStringSubmatch(found[i].str)
				//mainBag := otherMatches[1]
				split := SplitParse(otherMatches[2], ",")
				if InList(split, bagBuilder) {
					children := make([]*Node, len(split))
					for i, s := range split {
						children[i] = bagBuilder[s]
					}
					bagBuilder[otherMatches[1]] = &Node{bagId: bagMap[otherMatches[1]], name: otherMatches[1], children: children}
					items = append(items[:found[i].id], items[found[i].id+1:]...)
					bagCh <- otherMatches[1]
				}
			}
		}
		bagCh <- bag
		if len(items) == 0 {
			close(bagCh)
		}
	}
	count := 0
	for _, v := range bagBuilder {
		if v.FindName("shiny gold", true) != nil {
			count = count + 1
		}
	}
	fmt.Println(count)
}

type IdString struct {
	str string
	id  int
}

func GetMatching(list []string, match string) []IdString {
	ret := make([]IdString, 0)
	for i, item := range list {
		if strings.Contains(item, match) {
			ret = append(ret, IdString{str: item, id: i})
		}
	}
	return ret
}

func SplitParse(str, sep string) []string {
	split := strings.Split(str, sep)
	ret := make([]string, len(split))
	for i, s := range split {
		text := countRegex.FindStringSubmatch(strings.TrimSpace(s))[1]
		ret[i] = text
	}
	return ret
}

func InList(list []string, match map[string]*Node) bool {
	for _, m := range list {
		if _, ok := match[m]; !ok {
			return false
		}
	}
	return true
}
