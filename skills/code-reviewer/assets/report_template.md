<!-- Code Review Output Template -->

# 🛡️ Code Review & Security Audit Report

## Executive Summary
- **Target Inspected:** `{{TARGET_PATH_OR_DIFF}}`
- **Review Date:** `{{DATE}}`
- **Overall Verdict:** `{{PRODUCTION_READY | NEEDS_REVISION | CRITICAL_RISK}}`
- **DRY Compliance Score:** `{{SCORE}}/10`
- **Files Inspected:**
  - `{{FILE_1}}`
  - `{{FILE_2}}`

---

## 🚨 Critical & High Severity Findings

### [{{SEVERITY}}]: {{FINDING_TITLE}}
- **Location:** `{{FILE_PATH}}:L{{LINE_NUMBER}}`
- **Category:** `{{CATEGORY}}` (Memory Safety | Concurrency | DRY Violation | IPC Safety | Error Handling)
- **Problem & Root Cause:**
  {{EXPLANATION_OF_FLAW_AND_RISK}}

- **Actionable Drop-in Fix:**
```{{LANGUAGE}}
// Drop-in replacement implementation
```

---

## ⚠️ Medium & Low Severity Findings

### [{{SEVERITY}}]: {{FINDING_TITLE}}
- **Location:** `{{FILE_PATH}}:L{{LINE_NUMBER}}`
- **Recommendation:**
  {{IMPROVEMENT_DESCRIPTION}}

---

## 🧹 DRY & Architectural Consolidation
- **Duplicate Logic Identified:** {{SUMMARY_OF_DUPLICATION}}
- **Refactoring Strategy:** {{ACTIONABLE_ABSTRACTION_STEPS}}

---

## ✅ Positive Highlights & Clean Patterns
- {{NOTEWORTHY_SAFE_OR_ELEGANT_PATTERNS}}
