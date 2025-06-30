# Week 0 from ZKET core program
> Notes, takeaways and answers from the repo resources.

## Prerequisite Knowledge 

- ***What are Zero-Knowledge Proofs?***
    
    **Zero-Knowledge Proofs** are protocols/systems that can prove the validity of a statement (true|false) without revealing the actual statement. A Zero-Knowledge protocol are composed of two parties the prover, the party that is trying to claim the validity of statement and the verifier the party that verifies  the validity of this statement without taking knowledge of it.

- ***What are the principles of soundness, completeness, and zero-knowledge?***

    **soundness**, if the input is invalid, it's (except with negligible probability) impossible for a dishonest prover to convince the verifier that the statement is true. In other words, a false statement cannot be proven as true, except in extremely rare cases (e.g., with unbounded computational power).
    
    **completeness**, if the input is valid the the zk protocol will always return true

    **zero-knowledge**, the verifier learns nothing about the statement beyond its validity or falsity
    
- ***What distinguishes interactive from non-interactive proofs? :::***
     
    The interactive requires rounds of back and forth communication between the parties to complete the proof;
    Non-interactive, verify the validity of a statement in just one interaction, not requiring further interaction.

## Practical

**Rustlings**: https://github.com/PedroCo3lho/rustlings 

---

> TODO &darr;

### Thought Experiments
Engaging with these thought experiments will make abstract concepts more tangible and facilitate a deeper understanding of ZKPs. Each of these examples offers a unique perspective on the concept of zero-knowledge proofs:

Example 1: Where’s Waldo?
Example 2: Colored Balls (interactive)
Example 3: Sudoku (non-interactive)
:::info 🤔 Consider the following:

- ***Which example did you find most enlightening, and why?***
- ***How do these examples demonstrate the principles of zero-knowledge proofs?***
- ***Can you think of any potential applications of these concepts in everyday life?***
- ***Can you explain these thought experiments to someone non-technical? Try it with a friend or family member! :::***

### Use-Cases and Applications
To connect theory with practice, let's explore the practical applications of ZKPs. The following resources illustrate how ZKPs are employed in real-world applications, particularly in the domain of blockchain:

Ethereum Use Cases for ZKPs
More Applications
:::info 🤔 Consider the following:

- ***Which application of ZKP do you find most intriguing, and why?***
- ***Can you imagine any other potential applications of ZKP? :::***
