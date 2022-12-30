Feature: Intitial service function

    Scenario Outline: Adding transactions
        When service is started
        And a transaction with amount <amount>, and datetime <datetime> is added
        Then a transaction with id <id>, amount <amount>, and datetime <datetime> exists

        Examples:
        | id | amount | datetime            |
        | 0  | 105    | 06-07-2004 00:00:00 |
        | 0  | 20000  | 01-01-2023 01:59:59 |

    Scenario: Id increments with every added transaction
        When service is started
        And a transaction with amount 1000, and datetime 01-01-2000 00:00:00 is added
        And a transaction with amount 5050, and datetime 02-01-2000 00:00:00 is added
        Then a transaction with id 1, amount 5050, and datetime 02-01-2000 00:00:00 exists

    Scenario: Updating transaction has persistent effect
        When service is started
        And a transaction with amount 10050, and datetime 25-12-2020 12:00:00 is added
        Then a transaction with id 0, amount 10050, and datetime 25-12-2020 12:00:00 exists
        When a transaction is updated with values id 0, amount 10000 and datetime 25-12-2020 12:00:00
        Then a transaction with id 0, amount 10000, and datetime 25-12-2020 12:00:00 exists

    Scenario: Removing transaction has persistent effect
        When service is started
        And a transaction with amount 50, and datetime 01-01-2000 05:50:00 is added
        And a transaction with id 0 is removed
        Then a transaction with id 0 does not exist