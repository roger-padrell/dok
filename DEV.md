# Developing guide
## `.dok` file structure
```json
{
  "metadata":{
    "author":"author-name",
    "type":"project-type",
    "name":"project-name",
    "version":"project-version",
    "description":"project-description",
    "license":"project-license",
    "repository":"optional URL to git repo",
    "distributor":"optional link to language package manager's page for the project"
  },
  "dependencies":[
    "list of project dependencies",
    "in package-url",
    "(https://github.com/package-url/purl-spec)"
  ],
  "types":{ //a list of types/structs/interfaces/classes exported by the main file
    "something":{
      "definition":"full-type-definition",
      "description": "extracted from comments"
      "usage":"extracted from comments",
      "implementations":[
        "list of functions that implement",
        "utils for this type",
        "and link to where they're described in the documentation"
      ]
    }
  }
  "functions":{
    "definition": "function-definition",
    "description": "extracted from comments",
    "params":{
      "name":"param-name",
      "type":"optional param type",
      "description":"from comments",
      "default":"if it exists"
    }
    "examples": [
      {
        "name": "name",
        "description": "description",
        "code": "code in the example"
      }
    ]
  }
}
```
