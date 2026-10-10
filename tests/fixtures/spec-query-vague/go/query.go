package query

func ParseQuery(s string) []string {
	return strings.Fields(s)
}
