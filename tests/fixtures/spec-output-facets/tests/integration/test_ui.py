def test_title(page):
    page.goto("/")
    expect(page).to_have_title("calc")
