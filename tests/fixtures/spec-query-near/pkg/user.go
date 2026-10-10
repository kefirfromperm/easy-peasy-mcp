func Load(id string) *User {
	u, err := find(id)
	if err != nil {
		return nil
	}
	return u
}
