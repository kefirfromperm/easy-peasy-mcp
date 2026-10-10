func Save(o *Order) error {
	err := put(o)
	if err != nil {
		return fmt.Errorf("order: %w", err)
	}
	return nil
}
