# Evaluation Report Template

## Header

```markdown
# Evaluation Report

**Feature**: <feature name>
**Sprint Contract**: <link to contract>
**Date**: <YYYY-MM-DD>
**Evaluator**: evaluator agent
**Generator**: generator agent
```

## Summary

```markdown
## Summary

**Overall Grade**: <A/B/C/D> (<score>/100)

| Dimension | Score | Max |
|-----------|-------|-----|
| Functional Completeness | | 30 |
| Code Quality | | 25 |
| Test Coverage | | 20 |
| Architecture | | 15 |
| Safety | | 10 |
| **Total** | | **100** |

**Verdict**: <Approve for PR / Return for fixes / Major rework needed>
```

## Detailed Findings

```markdown
## Detailed Findings

### Strengths
- <What was done well>

### Issues
For each issue, provide:

#### Issue N: <Title>
- **Severity**: Critical / Major / Minor
- **Dimension**: <which evaluation dimension>
- **File(s)**: <file path(s) with line numbers>
- **Description**: <what's wrong>
- **Fix**: <specific, actionable fix>

### Missing Items
- <Acceptance criteria not met>
- <Edge cases not covered>
```

## Recommendation

```markdown
## Recommendation

<One of:>
- ✅ **Approve**: Ready for PR. All criteria met.
- 🔄 **Return**: Fix <N> issues and re-evaluate. Estimated effort: <low/medium/high>.
- ❌ **Rework**: Fundamental issues require significant changes. Re-negotiate contract scope.

### Next Steps
1. <Specific action item>
2. <Specific action item>
```

## Usage

1. Copy this template
2. Fill in the Header
3. Use `criteria-template.md` to score each dimension
4. Write detailed findings for any non-pass items
5. Calculate total score and assign grade:
   - A: 90-100 (Approve)
   - B: 75-89 (Approve with minor fixes)
   - C: 60-74 (Return for fixes)
   - D: < 60 (Rework)
6. Save to `production/session-logs/eval_<timestamp>.md`
