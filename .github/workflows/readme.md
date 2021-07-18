The following step should be the final step of all jobs. This enables SSHing into the job when it fails, or when desired during a manual run of the workflow.

```yaml
# Enable tmate debugging of manually-triggered workflows if the input option was provided
- name: Setup tmate session
  uses: mxschmitt/action-tmate@v3
  if: ${{ failure() || (github.event_name == 'workflow_dispatch' && github.event.inputs.debug_enabled) }}
```
