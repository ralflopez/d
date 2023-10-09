pub struct Pageable {
    pub page: i64,
    pub items_per_page: i64,
}

impl Pageable {
    pub fn new(page: i64, items_per_page: i64) -> Self {
        Self {
            page,
            items_per_page,
        }
    }

    pub fn offset(&self) -> i64 {
        if self.page < 1 {
            return 0;
        }

        (self.page - 1) * self.items_per_page
    }

    pub fn size(&self) -> i64 {
        self.items_per_page
    }
}
