'use client'

import 'github-markdown-css/github-markdown-light.css'
import { usePathname, useRouter } from 'next/navigation'
import Markdown from 'react-markdown'
import { formatDistance, fromUnixTime } from 'date-fns'
import styles from './CodeTable.module.css'
import { Space, Table, TableProps } from 'antd/lib'
import {
    FolderIcon,
    DocumentIcon,
} from '@heroicons/react/20/solid'

export interface DataType {
    oid: string;
    name: string;
    content_type: string;
    message: string;
    date: number;
}

const CodeTable = ({ directory, readmeContent }) => {
    const router = useRouter();
    const fileCodeContainerStyle = {
        width: '100%',
        margin: '0 auto',
        borderRadius: '0.5rem',
        marginTop: '10px'
    };
    const pathname = usePathname();
    let real_path = pathname.replace("/tree", "");
    var columns: TableProps<DataType>['columns'] = [
        {
            title: 'Name',
            dataIndex: ['name', 'content_type'],
            key: 'name',
            render: (_, record) => {
                return <>
                    {record.content_type === "file" &&
                        <Space>
                            <DocumentIcon className="size-6" />
                            <span onClick={() => handleFileClick(record)}>{record.name}</span>
                        </Space>
                    }
                    {record.content_type === "directory" &&
                        <Space>
                            <FolderIcon className="size-6" />
                            <a onClick={() => handleDirectoryClick(record)}>{record.name}</a>
                        </Space>}
                </>
            }
        },
        {
            title: 'Message',
            dataIndex: 'message',
            key: 'message',
            render: (text) => <a>{text}</a>,
        },
        {
            title: 'Date',
            dataIndex: 'date',
            key: 'date',
            render: (_, { date }) => (
                <>
                    {date && formatDistance(fromUnixTime(date), new Date(), { addSuffix: true })}
                </>
            )
        }
    ];

    const handleFileClick = (file) => {
        const newPath = `/blob/${real_path}/${file.name}`;
        router.push(newPath);
    };

    const handleDirectoryClick = async (directory) => {
        var newPath = '';
        if (real_path === '/') {
            newPath = `/tree/${directory.name}`;
        } else {
            newPath = `/tree/${real_path}/${directory.name}`;
        }
        router.push(
            newPath,
        );
    };

    const handleGoBack = () => {
        const safePath = real_path.split('/');

        if (safePath.length == 1) {
            router.push('/')
        } else {
            router.push(`/tree/${safePath.slice(0, -1).join('/')}`);
        }
    };

    // sort by file type, render folder type first
    const sortedDir = directory.sort((a, b) => {
        if (a.content_type === 'directory' && b.content_type === 'file') {
            return -1;
        } else if (a.content_type === 'file' && b.content_type === 'directory') {
            return 1;
        } else {
            return 0;
        }
    });


    return (
        <div style={fileCodeContainerStyle}>
            <Table style={{ clear: "none" }} rowClassName={styles.dirShowTr} pagination={false} columns={columns} dataSource={sortedDir} />
            {readmeContent && (
                <div className={styles.markdownContent}>
                    <div className="markdown-body">
                        <Markdown>{readmeContent}</Markdown>
                    </div>
                </div>
            )}
        </div>
    );
};



export default CodeTable;
