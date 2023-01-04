Feature: Intitial service function

    Scenario Outline: Adding transactions
        When service is started
        And a transaction with amount <amount>, and date "<date>" is added
        Then a transaction with id <id>, amount <amount>, and date "<date>" exists

        Examples:
        | id | amount | date       |
        | 0  | 105    | 06-07-2004 |
        | 0  | 20000  | 01-01-2023 |

    Scenario: Id increments with every added transaction
        When service is started
        And a transaction with amount 1000, and date "01-01-2000" is added
        And a transaction with amount 5050, and date "02-01-2000" is added
        Then a transaction with id 1, amount 5050, and date "02-01-2000" exists

    Scenario: Updating transaction has persistent effect
        When service is started
        And a transaction with amount 10050, and date "25-12-2020" is added
        Then a transaction with id 0, amount 10050, and date "25-12-2020" exists
        When a transaction is updated with values id 0, amount 10000 and date "25-12-2020"
        Then a transaction with id 0, amount 10000, and date "25-12-2020" exists

    Scenario: Removing transaction has persistent effect
        When service is started
        And a transaction with amount 50, and date "01-01-2000" is added
        And a transaction with id 0 is removed
        Then a transaction with id 0 does not exist