import { invoke } from '@tauri-apps/api/core';
import type { Agent } from '@/types';

interface AgentDto {
  agentId: string;
  name: string;
  description?: string;
  status?: string;
  workspacePath?: string;
  defaultModel?: string;
  createdAt?: string;
  updatedAt?: string;
}

function fromAgentDto(dto: AgentDto): Agent {
  return {
    agentId: dto.agentId,
    name: dto.name,
    description: dto.description ?? undefined,
    status: dto.status ?? undefined,
    workspacePath: dto.workspacePath ?? undefined,
    defaultModel: dto.defaultModel ?? undefined,
    createdAt: dto.createdAt ? new Date(dto.createdAt) : undefined,
    updatedAt: dto.updatedAt ? new Date(dto.updatedAt) : undefined,
  };
}

export const agentApi = {
  async listAgents(): Promise<Agent[]> {
    const agents = await invoke<AgentDto[]>('agent_list');
    return agents.map(fromAgentDto);
  },

  async getActiveAgent(): Promise<Agent> {
    const agent = await invoke<AgentDto>('agent_get_active');
    return fromAgentDto(agent);
  },

  async setActiveAgent(agentId: string): Promise<void> {
    return invoke('agent_set_active', { agentId });
  },

  async createAgent(agent: {
    agentId: string;
    name: string;
    description?: string;
    defaultModel?: string;
  }): Promise<void> {
    return invoke('agent_create', {
      agent: {
        agentId: agent.agentId,
        name: agent.name,
        description: agent.description,
        defaultModel: agent.defaultModel,
      },
    });
  },

  async deleteAgent(agentId: string): Promise<void> {
    return invoke('agent_delete', { agentId });
  },
};
