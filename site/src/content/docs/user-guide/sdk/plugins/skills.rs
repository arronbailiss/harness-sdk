#![allow(dead_code, deprecated)]

mod usage {
    // --8<-- [start:usage]
    use strands::vended_plugins::skills::{
        AgentSkills,
        AgentSkillsConfig,
        Skill,
        SkillConfig,
        SkillSource,
    };
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let custom = Skill::new(SkillConfig {
            name: "custom-greeting".into(),
            description: "Generate custom greetings".into(),
            instructions: Some("Greet the user by name.".into()),
            ..SkillConfig::default()
        });
        let sources = vec![
            SkillSource::from("./skills/"),
            SkillSource::from(custom),
        ];
        let plugin = AgentSkills::new(
            AgentSkillsConfig::builder().skills(sources).build(),
        );
        Agent::builder()
            .plugins([plugin])
            .printer(false)
            .build()
    }
    // --8<-- [end:usage]
}

mod tools {
    // --8<-- [start:tools]
    use strands::vended_plugins::skills::{
        AgentSkills,
        AgentSkillsConfig,
    };
    use strands::vended_tools::file_editor::file_editor;
    use strands::vended_tools::shell::{
        make_shell,
        MakeShellOptions,
    };
    use strands::Agent;

    fn agent() -> strands::Result<Agent> {
        let skills = AgentSkills::new(
            AgentSkillsConfig::builder()
                .skills(["./skills/"])
                .build(),
        );
        let shell = make_shell(MakeShellOptions::default())?;
        Agent::builder()
            .plugins([skills])
            .tools([file_editor(), shell])
            .printer(false)
            .build()
    }
    // --8<-- [end:tools]
}

mod programmatic {
    // --8<-- [start:programmatic]
    use strands::vended_plugins::skills::{
        Skill,
        SkillConfig,
        SkillLoadOptions,
    };

    fn load() -> strands::Result<Vec<Skill>> {
        let direct = Skill::new(SkillConfig {
            name: "code-review".into(),
            description: "Review code for bugs".into(),
            instructions: Some("Review the provided code.".into()),
            ..SkillConfig::default()
        });
        let parsed = Skill::from_content(
            "---\nname: summarize\ndescription: Summarize text\n---\nBe concise.",
            SkillLoadOptions::default(),
        )?;
        let mut skills = Skill::from_directory(
            "./skills/",
            SkillLoadOptions::default(),
        )?;
        skills.extend([direct, parsed]);
        Ok(skills)
    }
    // --8<-- [end:programmatic]
}

mod runtime {
    // --8<-- [start:runtime]
    use strands::vended_plugins::skills::{
        AgentSkills,
        AgentSkillsConfig,
        Skill,
        SkillConfig,
    };
    use strands::Agent;

    async fn update(
        plugin: &AgentSkills,
        agent: &Agent,
    ) -> strands::Result<Vec<String>> {
        let summarize = Skill::new(SkillConfig {
            name: "summarize".into(),
            description: "Summarize long documents".into(),
            instructions: Some("Produce a concise summary.".into()),
            ..SkillConfig::default()
        });
        plugin.set_available_skills([summarize]);
        let _available = plugin
            .get_available_skills(Some(agent.local()))
            .await;
        Ok(plugin.get_activated_skills(agent.local()))
    }

    fn plugin() -> AgentSkills {
        AgentSkills::new(
            AgentSkillsConfig::builder()
                .skills(["./skills/"])
                .build(),
        )
    }
    // --8<-- [end:runtime]
}
