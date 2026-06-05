pub const DEFAULT_CONFIG: &str = r#"
# Types de champs disponibles : text, number, select
# Les clés définies ici doivent correspondre aux variables dans tes formats : {ma_cle}

# [[fields]]
# key      = "ticket"
# label    = "Numéro de ticket"
# type     = "number"
# required = true

# [[fields]]
# key      = "description"
# label    = "Description"
# type     = "text"
# required = true

# [[fields]]
# key    = "type"
# label  = "Type"
# type   = "select"
# values = ["feat", "fix", "refactor", "docs", "chore"]
# required = true

# [formats]
# branch   = "{type}/{ticket}-{description}"
# commit   = "{type}: {description}"
# pr_title = "[{ticket}] {description}"
"#;