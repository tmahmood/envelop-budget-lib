# Envelop budget lib

A library that uses envelop budgeting to manage expenses.

This library intends to provide APIs to develop a envelop budgeting application. It's heavily in development. I am making sure of the correctness and reliability of the API with as many unit and behavioral tests as possible. It uses SQLite database.

## Features
- [x] Multiple accounts
- [x] Categories
- [x] Transactions grouped by categories and accounts
- [x] All incomes are automatically ready to assign
- [.] Envelop budgeting
    - [x] Allocating
    - [x] Funding
    - [x] Transferring funds
    - [ ] Cleaning up
- [ ] Reporting

## Work in progress

-[ ] Rewriting some core calculations.
-[ ] Category calculation should not be dependent on budget account.
-[ ] Categories are only going to be about grouping the expenses,
-[ ] And all the income will go to default category.

### Calculating actual_total_balance

Default category will keep track of actual money. All the income/expenses will be used to calculate actual_total_balance, regardless of category. As actual_total_balance is calculated only from income and expenses, we have one and only source of truth for actual money available.

`actual_total_balance = total income - total expenses`

### Calculating available money to fund changes planned
I will keep track of category funding by transfer in/out. So when we are calculating available money to fund, we can sum all the transfer outs from default category and minus it from `actual_total_balance`. We need to consider how much money we actually have.

`available_to_fund = actual_total_balance - total transfer out from default category`
