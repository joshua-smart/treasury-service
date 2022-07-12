Feature: Test feature

    Scenario: This is a test feature
        Given the number <n> is in the state
        When the addTwo method is called
        Then the value in state is <result>
        Examples:
            | n   | result |
            | 1   | 3      |
            | -10 | -8     |
